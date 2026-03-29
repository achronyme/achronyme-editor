use crate::convert;
use crate::document::DocumentStore;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
    client: Client,
    documents: DocumentStore,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DocumentStore::new(),
        }
    }

    async fn publish_diagnostics_for(&self, uri: Uri, text: &str) {
        let (_program, errors) = achronyme_parser::parse_program(text);
        let diagnostics: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, text)
            .into_iter()
            .map(convert::diagnostic)
            .collect();

        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}

impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp_server::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                document_symbol_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "ach-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "ach-lsp initialized")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp_server::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.documents.open(&uri.to_string(), text.clone());
        self.publish_diagnostics_for(uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().last() {
            let uri = params.text_document.uri;
            self.documents.update(&uri.to_string(), change.text.clone());
            self.publish_diagnostics_for(uri, &change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.close(&params.text_document.uri.to_string());
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp_server::jsonrpc::Result<Option<Hover>> {
        let uri = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let pos = params.text_document_position_params.position;

        let text = match self.documents.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };

        let (word, core_range) =
            match ach_lsp_core::document::word_at_position(&text, pos.line, pos.character) {
                Some(w) => w,
                None => return Ok(None),
            };

        let doc = match ach_lsp_core::hover::hover_for(&word) {
            Some(d) => d,
            None => return Ok(None),
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: doc.to_string(),
            }),
            range: Some(convert::range(core_range)),
        }))
    }

    async fn completion(
        &self,
        _: CompletionParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<CompletionResponse>> {
        let mut items: Vec<CompletionItem> = ach_lsp_core::completion::keyword_completions()
            .into_iter()
            .map(convert::completion_item)
            .collect();
        items.extend(
            ach_lsp_core::completion::snippet_completions()
                .into_iter()
                .map(convert::completion_item),
        );
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let uri_str = params
            .text_document_position_params
            .text_document
            .uri
            .to_string();
        let pos = params.text_document_position_params.position;

        let text = match self.documents.get(&uri_str) {
            Some(t) => t,
            None => return Ok(None),
        };

        let byte_offset =
            match ach_lsp_core::definitions::position_to_byte_offset(&text, pos.line, pos.character)
            {
                Some(o) => o,
                None => return Ok(None),
            };

        let r = match ach_lsp_core::definitions::goto_definition(&text, byte_offset) {
            Some(r) => r,
            None => return Ok(None),
        };

        let uri = params.text_document_position_params.text_document.uri;
        Ok(Some(GotoDefinitionResponse::Scalar(Location::new(
            uri,
            convert::range(r),
        ))))
    }

    async fn references(
        &self,
        params: ReferenceParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<Vec<Location>>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;

        let text = match self.documents.get(&uri.to_string()) {
            Some(t) => t,
            None => return Ok(None),
        };

        let byte_offset =
            match ach_lsp_core::definitions::position_to_byte_offset(&text, pos.line, pos.character)
            {
                Some(o) => o,
                None => return Ok(None),
            };

        let ranges = ach_lsp_core::definitions::find_references(&text, byte_offset);
        if ranges.is_empty() {
            return Ok(None);
        }

        let locations: Vec<Location> = ranges
            .into_iter()
            .map(|r| Location::new(uri.clone(), convert::range(r)))
            .collect();
        Ok(Some(locations))
    }

    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<PrepareRenameResponse>> {
        let uri_str = params.text_document.uri.to_string();
        let pos = params.position;

        let text = match self.documents.get(&uri_str) {
            Some(t) => t,
            None => return Ok(None),
        };

        let byte_offset =
            match ach_lsp_core::definitions::position_to_byte_offset(&text, pos.line, pos.character)
            {
                Some(o) => o,
                None => return Ok(None),
            };

        match ach_lsp_core::definitions::prepare_rename(&text, byte_offset) {
            Some((r, _)) => Ok(Some(PrepareRenameResponse::Range(convert::range(r)))),
            None => Ok(None),
        }
    }

    async fn rename(
        &self,
        params: RenameParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<WorkspaceEdit>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let new_name = params.new_name;

        let text = match self.documents.get(&uri.to_string()) {
            Some(t) => t,
            None => return Ok(None),
        };

        let byte_offset =
            match ach_lsp_core::definitions::position_to_byte_offset(&text, pos.line, pos.character)
            {
                Some(o) => o,
                None => return Ok(None),
            };

        let edits = ach_lsp_core::definitions::rename(&text, byte_offset, &new_name);
        if edits.is_empty() {
            return Ok(None);
        }

        let text_edits: Vec<TextEdit> = edits.into_iter().map(convert::text_edit).collect();

        let mut changes = std::collections::HashMap::new();
        changes.insert(uri, text_edits);

        Ok(Some(WorkspaceEdit {
            changes: Some(changes),
            ..Default::default()
        }))
    }

    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<DocumentSymbolResponse>> {
        let uri = params.text_document.uri.to_string();
        let text = match self.documents.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        let syms: Vec<DocumentSymbol> = ach_lsp_core::symbols::document_symbols(&text)
            .into_iter()
            .map(convert::document_symbol)
            .collect();
        Ok(Some(DocumentSymbolResponse::Nested(syms)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_source_produces_no_diagnostics() {
        let text = "let x = 1 + 2";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, text)
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert!(diags.is_empty());
    }

    #[test]
    fn single_error_maps_correctly() {
        let text = "let x =";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, text)
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert!(!diags.is_empty(), "should report at least one error");
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diags[0].source.as_deref(), Some("ach"));
    }

    #[test]
    fn multiple_errors_all_reported() {
        let text = "let x =\nlet y =";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, text)
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert!(
            diags.len() >= 2,
            "expected at least 2 diagnostics, got {}",
            diags.len()
        );
    }

    #[test]
    fn line_col_converted_to_zero_based() {
        use achronyme_parser::diagnostic::SpanRange;
        let errors = vec![achronyme_parser::Diagnostic::error(
            "test error",
            SpanRange::new(0, 5, 3, 7, 3, 12),
        )];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, "")
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(diags[0].range.start.line, 2);
        assert_eq!(diags[0].range.start.character, 6);
        assert_eq!(diags[0].range.end.line, 2);
        assert_eq!(diags[0].range.end.character, 11);
    }

    #[test]
    fn point_span_extends_to_end_of_line() {
        use achronyme_parser::diagnostic::SpanRange;
        let text = "let x = ;";
        let errors = vec![achronyme_parser::Diagnostic::error(
            "expected expression",
            SpanRange::point(1, 9, 8),
        )];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, text)
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(diags[0].range.start.character, 8);
        assert_eq!(diags[0].range.end.character, text.len() as u32);
    }

    #[test]
    fn severity_mapping() {
        use achronyme_parser::diagnostic::SpanRange;
        let make = |sev: achronyme_parser::Severity| {
            let mut d = achronyme_parser::Diagnostic::error("x", SpanRange::point(1, 1, 0));
            d.severity = sev;
            d
        };

        let errors = vec![
            make(achronyme_parser::Severity::Error),
            make(achronyme_parser::Severity::Warning),
            make(achronyme_parser::Severity::Note),
            make(achronyme_parser::Severity::Help),
        ];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, "")
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diags[1].severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(diags[2].severity, Some(DiagnosticSeverity::INFORMATION));
        assert_eq!(diags[3].severity, Some(DiagnosticSeverity::HINT));
    }

    #[test]
    fn diagnostic_code_preserved() {
        use achronyme_parser::diagnostic::SpanRange;
        let errors =
            vec![
                achronyme_parser::Diagnostic::warning("unused", SpanRange::point(1, 1, 0))
                    .with_code("W001"),
            ];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, "")
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(
            diags[0].code,
            Some(NumberOrString::String("W001".to_string()))
        );
    }

    #[test]
    fn diagnostic_without_code() {
        use achronyme_parser::diagnostic::SpanRange;
        let errors = vec![achronyme_parser::Diagnostic::error(
            "oops",
            SpanRange::point(1, 1, 0),
        )];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, "")
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(diags[0].code, None);
    }

    #[test]
    fn message_preserved() {
        use achronyme_parser::diagnostic::SpanRange;
        let errors = vec![achronyme_parser::Diagnostic::error(
            "undefined variable: `foo`",
            SpanRange::point(1, 1, 0),
        )];
        let diags: Vec<Diagnostic> = ach_lsp_core::diagnostics::map_diagnostics(&errors, "")
            .into_iter()
            .map(convert::diagnostic)
            .collect();
        assert_eq!(diags[0].message, "undefined variable: `foo`");
    }
}
