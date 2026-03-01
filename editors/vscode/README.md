# Achronyme for VS Code

Language support for [Achronyme](https://github.com/achronyme/achronyme) — a zero-knowledge cryptography DSL with native proof generation.

## Features

### Syntax Highlighting

Full TextMate grammar for `.ach` files:

- Keywords: `let`, `mut`, `fn`, `if`, `else`, `while`, `for`, `prove`, `witness`, `public`, `import`, `export`, ...
- Literals: integers, field elements (`0p...`), big integers (`0i256...`, `0i512...`), strings
- All 32 builtin functions highlighted distinctly
- Import statements with module alias highlighting

### Diagnostics

Real-time parse error diagnostics as you type — no save required.

### Hover Documentation

Hover over any keyword or builtin function to see inline documentation with signatures and examples.

### Autocompletion

- Keywords and snippets (`fn`, `prove {}`, `for..in`, `if/else`, `while`)
- All 32 builtin functions with argument tab-stops

### Auto-download CLI

The extension can automatically download the `ach` CLI from GitHub Releases if it's not found on your system. It also checks for updates on each activation.

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `achronyme.executablePath` | `""` | Path to the `ach` CLI binary. Leave empty for auto-detection or auto-download. |
| `achronyme.lspPath` | `""` | Path to the `ach-lsp` binary. Leave empty to use the bundled binary. |

## Requirements

- VS Code 1.85+
- The `ach-lsp` binary is bundled in platform-specific VSIX releases (no manual install needed)

## Quick Start

1. Install the extension
2. Open any `.ach` file
3. If the `ach` CLI is not installed, the extension will offer to download it

```ach
import "./math.ach" as math

fn main() {
    let x = 42
    print(math.add(x, 1))

    prove {
        public out
        witness secret
        assert_eq(poseidon(secret), out)
    }
}
```

## Links

- [Achronyme Language](https://github.com/achronyme/achronyme) — compiler, VM, and prover
- [Report Issues](https://github.com/achronyme/achronyme-editor/issues)

## License

GPL-3.0
