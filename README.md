# Achronyme Editor

[![CI](https://github.com/achronyme/achronyme-editor/actions/workflows/ci.yml/badge.svg)](https://github.com/achronyme/achronyme-editor/actions/workflows/ci.yml)

Editor tooling for the [Achronyme](https://github.com/achronyme/achronyme) ZK programming language.

---

## Features

- **Syntax highlighting** — TextMate grammar for `.ach` files: keywords, field/bigint literals, strings, comments, operators
- **Parse error diagnostics** — Real-time squiggles with error messages as you type
- **Hover documentation** — Hover over keywords and builtin functions for inline docs with signatures

---

## Installation

### VS Code (development mode)

```bash
# Build the language server
cargo build --release -p ach-lsp

# Copy the binary where the extension expects it
mkdir -p editors/vscode/bin
cp target/release/ach-lsp editors/vscode/bin/

# Install extension dependencies
cd editors/vscode && npm install && npm run build

# Launch VS Code with the extension
code --extensionDevelopmentPath=editors/vscode
```

Open any `.ach` file to get syntax highlighting, diagnostics, and hover.

### Custom LSP binary path

If `ach-lsp` is installed elsewhere (e.g. via `cargo install`), set `achronyme.lspPath` in VS Code settings:

```json
{
  "achronyme.lspPath": "/path/to/ach-lsp"
}
```

---

## Project Structure

```
achronyme-editor/
├── ach-lsp/                   Language server (Rust)
│   └── src/
│       ├── main.rs            Tokio + tower-lsp-server stdio setup
│       ├── backend.rs         LanguageServer trait implementation
│       ├── document.rs        Document store + text utilities
│       └── hover.rs           Static hover table (keywords + builtins)
├── editors/vscode/            VS Code extension
│   ├── src/extension.ts       LSP client, launches ach-lsp
│   ├── syntaxes/              TextMate grammar
│   └── language-configuration.json
└── .github/workflows/ci.yml
```

---

## Development

```bash
# Format + lint + build + test
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo build --workspace
cargo test --workspace
```

---

## License

GPL-3.0
