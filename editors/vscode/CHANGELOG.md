# Changelog

All notable changes to the Achronyme VS Code extension are listed here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.2.0-beta.21]

Tracks the language repo's `0.1.0-beta.21` release. No editor-facing
behavior changes; the bump exists to pull in the upstream walker fix
that unblocks SHA-256 imported from circomlib in `.ach prove` blocks
(see the language repo's release notes for context).

### Changed

- Bumped editor crates and the extension to `0.2.0-beta.21`. Cargo
  refs still pin `branch = "main"`; `Cargo.lock` refresh moves the
  resolved revision from beta.20 to the beta.21 release commit.

## [0.2.0-beta.20]

First Marketplace release. Brings circom support to parity with the
existing `.ach` story and updates the LSP to the beta.20 feature set.

### Added

- `.circom` language contribution — extension dispatch on `.circom`
  filenames, dedicated TextMate grammar, language configuration with
  comment / bracket rules.
- Circom diagnostics surfaced through `ach-lsp`:
  - Parser errors (E300-E306) and warnings.
  - Constraint analyzer codes E100-E102 (under-constrained signals),
    W101-W103 (double assignment, unused inputs).
  - Lowering errors (E200-E211) on self-contained sources — multi-file
    workspaces defer to the workspace-aware compile.
- Hover docs for circom keywords (`template`, `signal`, `pragma`,
  `component`, `include`, `function`, `var`) and the circomlib
  templates verified through the achronyme front-end (`Num2Bits`,
  `LessThan`, `IsZero`, `Poseidon`, `MiMCSponge`, `Pedersen`,
  `EdDSAPoseidonVerifier`, `Sha256`).
- Autocomplete for `.circom`: keywords, the three constraint
  operators (`<==`, `==>`, `<--`, `===`), the `template` /
  `signal input` / `signal output` / `include` snippets, and the
  circomlib component instantiations with canonical parameter shapes.
- Independent hover and completion tables per language so the same
  identifier (`Poseidon` vs `poseidon`) surfaces the right docs for
  the file you are editing instead of conflating the achronyme
  builtin with the circomlib component.

### Changed

- Re-licensed from GPL-3.0 to Apache-2.0 across the repository to
  match the rest of the Achronyme project.
- Bumped editor crates and the extension to `0.2.0-beta.20` so the
  VSIX matches the LSP capabilities shipped in the language repo.

### Fixed

- Hover docs for `mut` no longer inherit from `let` — `mut` is a
  standalone keyword with its own documentation.
- LSP core caught up with `Expr` AST shape changes in
  `achronyme-parser` so diagnostics work against current `.ach`
  syntax.
- Cleared clippy warnings in the `definitions` and `symbols`
  modules.

## [0.2.0-beta.18]

Internal pre-release; not published to the Marketplace. The full
feature set landed in `0.2.0-beta.20`.
