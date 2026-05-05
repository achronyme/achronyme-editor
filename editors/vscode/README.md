# Achronyme for VS Code

Language support for [Achronyme](https://achrony.me) — a Rust-based zero-knowledge cryptography DSL with native R1CS/Groth16 + Plonkish/Halo2 proof generation and a circom interop front-end.

Edits both `.ach` (Achronyme) and `.circom` (Circom) source.

## Features

### Syntax highlighting

TextMate grammars for both languages, with circomlib templates (`Num2Bits`, `Poseidon`, `MiMCSponge`, `EdDSAPoseidonVerifier`, `Sha256`, …) and the constraint operators (`<==`, `==>`, `<--`, `===`) tokenised distinctly so constraint-emitting lines stand out from witness hints.

### Diagnostics

- **`.ach`** — parser errors, warnings (W001/W002), `did you mean?` suggestions.
- **`.circom`** — parser errors (E300-E306), constraint analyzer (E100-E102, W101-W103: under-constrained signals, double assignments, unused inputs), and lowering errors (E200-E211) on self-contained sources. Multi-file projects defer lowering to the workspace-aware compile.

All published in real time as you type — no save required.

### Hover documentation

Hover over any keyword, builtin, method, type, or circomlib template to see signatures and inline examples. The two languages have distinct hover tables — `Poseidon` inside a `.circom` template surfaces the circomlib component docs, while `poseidon` inside an `.ach` `circuit` block surfaces the achronyme builtin.

### Autocompletion

- **`.ach`** — 27 keywords, 16 globals, 47 prototype methods, 6 statics (`Int::MAX`, `Field::ORDER`, …), and 12 code snippets (`fn`, `prove {}`, `circuit`, `for..in`, `if/else`, …).
- **`.circom`** — 16 keywords, plus snippets for `template`, `signal input/output`, `pragma`, `include`, the constraint operators, and the verified circomlib templates with their canonical parameter shapes.

### Run button

A play button appears in the editor title bar when a `.ach` file is open. Click it to execute with `ach run` in an integrated terminal. Auto-saves before running.

### Auto-download CLI

The extension can automatically download the `ach` CLI from GitHub Releases on first use if it's not found on your system, and checks for updates on each activation.

## Configuration

| Setting | Default | Description |
|---------|---------|-------------|
| `achronyme.executablePath` | `""` | Path to the `ach` CLI binary. Leave empty for auto-detection or auto-download. |
| `achronyme.lspPath` | `""` | Path to the `ach-lsp` binary. Leave empty to use the bundled binary. |

## Requirements

- VS Code 1.85+
- The `ach-lsp` binary is bundled in the platform-specific VSIX (linux-x64, darwin-x64, darwin-arm64, win32-x64) — no manual install needed.

## Quick start

1. Install the extension.
2. Open any `.ach` or `.circom` file.
3. If the `ach` CLI is not installed, the extension will offer to download it.

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

## Try it without installing

The browser playground at [play.achrony.me](https://play.achrony.me) ships the same diagnostics, hover, and autocomplete via WASM — useful for a quick look before installing locally.

## Links

- [Project site](https://achrony.me) — docs, tutorials, architecture reference.
- [Achronyme language](https://github.com/achronyme/achronyme) — compiler, VM, prover.
- [Report issues](https://github.com/achronyme/achronyme-editor/issues)

## License

Apache-2.0 — see the [LICENSE](https://github.com/achronyme/achronyme-editor/blob/main/LICENSE) in the repository root.
