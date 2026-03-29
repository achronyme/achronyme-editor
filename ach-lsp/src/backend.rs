use crate::completion;
use crate::document::{self, DocumentStore};
use crate::hover;
use crate::symbols;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
    client: Client,
    documents: DocumentStore,
}

/// Convert parser diagnostics to LSP diagnostics.
///
/// The parser uses 1-based line/col; LSP uses 0-based. Point spans (where
/// start == end) are extended to the end of the line for visibility in the
/// editor.
pub fn map_diagnostics(errors: &[achronyme_parser::Diagnostic], text: &str) -> Vec<Diagnostic> {
    errors
        .iter()
        .map(|e| {
            let span = &e.primary_span;
            let start_line = span.line_start.saturating_sub(1) as u32;
            let start_col = span.col_start.saturating_sub(1) as u32;
            let end_line = span.line_end.saturating_sub(1) as u32;
            let end_col = if span.col_end > span.col_start || span.line_end > span.line_start {
                span.col_end.saturating_sub(1) as u32
            } else {
                // Point span — extend to end of line for visibility
                text.lines()
                    .nth(start_line as usize)
                    .map(|l| l.len() as u32)
                    .unwrap_or(start_col + 1)
            };

            let severity = match e.severity {
                achronyme_parser::Severity::Error => DiagnosticSeverity::ERROR,
                achronyme_parser::Severity::Warning => DiagnosticSeverity::WARNING,
                achronyme_parser::Severity::Note => DiagnosticSeverity::INFORMATION,
                achronyme_parser::Severity::Help => DiagnosticSeverity::HINT,
            };

            Diagnostic {
                range: Range {
                    start: Position::new(start_line, start_col),
                    end: Position::new(end_line, end_col),
                },
                severity: Some(severity),
                code: e.code.as_ref().map(|c| NumberOrString::String(c.clone())),
                source: Some("ach".into()),
                message: e.message.clone(),
                ..Default::default()
            }
        })
        .collect()
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
        let diagnostics = map_diagnostics(&errors, text);

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

        let (word, range) = match document::word_at_position(&text, pos.line, pos.character) {
            Some(w) => w,
            None => return Ok(None),
        };

        let doc = match hover::hover_for(&word) {
            Some(d) => d,
            None => return Ok(None),
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: doc.to_string(),
            }),
            range: Some(range),
        }))
    }

    async fn completion(
        &self,
        _: CompletionParams,
    ) -> tower_lsp_server::jsonrpc::Result<Option<CompletionResponse>> {
        let mut items = completion::keyword_completions();
        items.extend(completion::snippet_completions());
        Ok(Some(CompletionResponse::Array(items)))
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
        let syms = symbols::document_symbols(&text);
        Ok(Some(DocumentSymbolResponse::Nested(syms)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use achronyme_parser::diagnostic::SpanRange;

    #[test]
    fn valid_source_produces_no_diagnostics() {
        let text = "let x = 1 + 2";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags = map_diagnostics(&errors, text);
        assert!(diags.is_empty());
    }

    #[test]
    fn single_error_maps_correctly() {
        let text = "let x =";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags = map_diagnostics(&errors, text);
        assert!(!diags.is_empty(), "should report at least one error");
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diags[0].source.as_deref(), Some("ach"));
    }

    #[test]
    fn multiple_errors_all_reported() {
        // Two broken statements — parser should recover and report both
        let text = "let x =\nlet y =";
        let (_prog, errors) = achronyme_parser::parse_program(text);
        let diags = map_diagnostics(&errors, text);
        assert!(
            diags.len() >= 2,
            "expected at least 2 diagnostics, got {}",
            diags.len()
        );
    }

    #[test]
    fn line_col_converted_to_zero_based() {
        // Parser uses 1-based, LSP uses 0-based
        let errors = vec![achronyme_parser::Diagnostic::error(
            "test error",
            SpanRange::new(0, 5, 3, 7, 3, 12),
        )];
        let diags = map_diagnostics(&errors, "");
        assert_eq!(diags[0].range.start.line, 2); // 3 - 1
        assert_eq!(diags[0].range.start.character, 6); // 7 - 1
        assert_eq!(diags[0].range.end.line, 2);
        assert_eq!(diags[0].range.end.character, 11); // 12 - 1
    }

    #[test]
    fn point_span_extends_to_end_of_line() {
        let text = "let x = ;";
        let errors = vec![achronyme_parser::Diagnostic::error(
            "expected expression",
            SpanRange::point(1, 9, 8),
        )];
        let diags = map_diagnostics(&errors, text);
        // Point span: col_end == col_start, so should extend to end of line
        assert_eq!(diags[0].range.start.character, 8); // col 9 -> 0-based 8
        assert_eq!(diags[0].range.end.character, text.len() as u32); // end of line
    }

    #[test]
    fn severity_mapping() {
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
        let diags = map_diagnostics(&errors, "");
        assert_eq!(diags[0].severity, Some(DiagnosticSeverity::ERROR));
        assert_eq!(diags[1].severity, Some(DiagnosticSeverity::WARNING));
        assert_eq!(diags[2].severity, Some(DiagnosticSeverity::INFORMATION));
        assert_eq!(diags[3].severity, Some(DiagnosticSeverity::HINT));
    }

    #[test]
    fn diagnostic_code_preserved() {
        let errors =
            vec![
                achronyme_parser::Diagnostic::warning("unused", SpanRange::point(1, 1, 0))
                    .with_code("W001"),
            ];
        let diags = map_diagnostics(&errors, "");
        assert_eq!(
            diags[0].code,
            Some(NumberOrString::String("W001".to_string()))
        );
    }

    #[test]
    fn diagnostic_without_code() {
        let errors = vec![achronyme_parser::Diagnostic::error(
            "oops",
            SpanRange::point(1, 1, 0),
        )];
        let diags = map_diagnostics(&errors, "");
        assert_eq!(diags[0].code, None);
    }

    #[test]
    fn message_preserved() {
        let errors = vec![achronyme_parser::Diagnostic::error(
            "undefined variable: `foo`",
            SpanRange::point(1, 1, 0),
        )];
        let diags = map_diagnostics(&errors, "");
        assert_eq!(diags[0].message, "undefined variable: `foo`");
    }
}
