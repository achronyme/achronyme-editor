use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer};

pub struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    async fn publish_diagnostics_for(&self, uri: Uri, text: &str) {
        let diagnostics = match achronyme_parser::parse_program_with_errors(text) {
            Ok(_) => vec![],
            Err(e) => {
                let line = e.line.saturating_sub(1) as u32;
                let col = e.col.saturating_sub(1) as u32;
                // Mark from error column to end of the offending line.
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
        self.publish_diagnostics_for(params.text_document.uri, &params.text_document.text)
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        // FULL sync: the last content change contains the entire document.
        if let Some(change) = params.content_changes.into_iter().last() {
            self.publish_diagnostics_for(params.text_document.uri, &change.text)
                .await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Clear diagnostics when the file is closed.
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }
}
