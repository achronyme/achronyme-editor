# Achronyme Editor

[![CI](https://github.com/achronyme/achronyme-editor/actions/workflows/ci.yml/badge.svg)](https://github.com/achronyme/achronyme-editor/actions/workflows/ci.yml)

Editor tooling for [Achronyme](https://github.com/achronyme/achronyme), a programming language for zero-knowledge circuits.

---

## Features

- **Syntax highlighting** — TextMate grammar for `.ach` files: keywords, field/bigint literals, strings, comments, operators, selective imports, export lists
- **Parse error diagnostics** — Real-time squiggles with error messages as you type
- **Hover documentation** — Hover over keywords and builtin functions for inline docs with signatures
- **Autocompletion** — Keywords, 43 builtin functions with argument tab-stops, and code snippets (fn, prove, for, if/else, while)
- **Run button** — Play button in the editor title bar to execute `.ach` files via `ach run` in an integrated terminal
- **Auto-download CLI** — Downloads `ach` from GitHub Releases if not found; checks for updates on each activation

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
│       ├── hover.rs           Static hover table (keywords + builtins)
│       └── completion.rs      Keyword, builtin, and snippet completions
├── editors/vscode/            VS Code extension
│   ├── src/
│   │   ├── extension.ts       LSP client, Run command, launches ach-lsp
│   │   └── download.ts        Auto-download/update ach CLI from GitHub Releases
│   ├── syntaxes/              TextMate grammar
│   └── language-configuration.json
└── .github/workflows/ci.yml
```

---

## Supported Syntax

The TextMate grammar highlights:

| Element | Examples |
|---------|---------|
| Keywords | `fn`, `let`, `mut`, `if`, `else`, `for`, `while`, `return`, `prove` |
| Control | `import`, `export`, `as`, `from` (contextual), `break`, `continue` |
| Imports | `import "path" as alias`, `import { x, y } from "path"` |
| Exports | `export fn`, `export let`, `export { x, y }` |
| Literals | `42`, `0pxFF`, `0i256d42`, `"string"`, `true`, `false`, `nil` |
| Builtins | `print`, `poseidon`, `assert`, `map`, `filter`, `reduce`, ... (43 total) |

---

## Development

```bash
# Format + lint + build + test (Rust)
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo build --workspace
cargo test --workspace

# Build + typecheck (VS Code extension)
cd editors/vscode
npm run build
npm run typecheck
```

---

## License

GPL-3.0
