import * as path from "path";
import {
  commands,
  ExtensionContext,
  workspace,
  window,
} from "vscode";
import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
} from "vscode-languageclient/node";
import { ensureAchBinary, getAchBinaryPath, findOnPath } from "./download";

let client: LanguageClient | undefined;

export async function activate(context: ExtensionContext): Promise<void> {
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

  // Check for ach CLI binary (non-blocking)
  ensureAchBinary().catch(() => {});

  // Register "Run Achronyme File" command
  context.subscriptions.push(
    commands.registerCommand("achronyme.run", async () => {
      const editor = window.activeTextEditor;
      if (!editor) {
        window.showErrorMessage("No active file to run.");
        return;
      }

      const document = editor.document;
      if (document.isDirty) {
        await document.save();
      }

      const filePath = document.uri.fsPath;

      // Resolve ach binary path
      let achPath = getAchBinaryPath() ?? (await findOnPath("ach"));
      if (!achPath) {
        const choice = await window.showErrorMessage(
          "Achronyme CLI (ach) not found. Download it or configure a path.",
          "Download",
          "Configure Path",
        );
        if (choice === "Download") {
          await ensureAchBinary();
          achPath = getAchBinaryPath();
        } else if (choice === "Configure Path") {
          await commands.executeCommand(
            "workbench.action.openSettings",
            "achronyme.executablePath",
          );
        }
        if (!achPath) return;
      }

      const terminal =
        window.terminals.find((t) => t.name === "Achronyme") ??
        window.createTerminal("Achronyme");
      terminal.show();
      terminal.sendText(`${achPath} run "${filePath}"`);
    }),
  );
}

export function deactivate(): Thenable<void> | undefined {
  return client?.stop();
}
