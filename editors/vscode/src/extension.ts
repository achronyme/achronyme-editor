import * as path from "path";
import {
  ExtensionContext,
  workspace,
  window,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(context: ExtensionContext): void {
  // Look for the ach-lsp binary:
  // 1. User-configured path
  // 2. Bundled in the extension's bin/ directory
  // 3. On PATH
  const config = workspace.getConfiguration("achronyme");
  const configuredPath = config.get<string>("lspPath");
  const serverCommand =
    configuredPath ||
    path.join(context.extensionPath, "bin", "ach-lsp");

  const serverOptions: ServerOptions = {
    command: serverCommand,
    args: [],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "achronyme" }],
  };

  client = new LanguageClient(
    "ach-lsp",
    "Achronyme Language Server",
    serverOptions,
    clientOptions,
  );

  client.start().catch((err) => {
    window.showWarningMessage(
      `ach-lsp failed to start: ${err}. Install it with: cargo install --path ach-lsp`,
    );
  });
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
