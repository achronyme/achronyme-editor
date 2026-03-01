use crate::completion;
use crate::document::{self, DocumentStore};
use crate::hover;
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
        let diagnostics = match achronyme_parser::parse_program_with_errors(text) {
            Ok(_) => vec![],
            Err(e) => {
                let line = e.line.saturating_sub(1) as u32;
                let col = e.col.saturating_sub(1) as u32;
                let end_col = text
                    .lines()
                    .nth(line as usize)
                    .map(|l| l.len() as u32)
                    .unwrap_or(col + 1);

                vec![Diagnostic {
                    range: Range {
                        start: Position::new(line, col),
                        end: Position::new(line, end_col),
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("ach".into()),
                    message: e.message,
                    ..Default::default()
                }]
            }
        };

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
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "ach-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
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
}
