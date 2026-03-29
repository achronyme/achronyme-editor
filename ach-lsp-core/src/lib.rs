//! Core LSP logic for Achronyme.
//!
//! This crate contains the pure logic for diagnostics, completions,
//! hover documentation, go-to-definition, references, rename, and
//! document symbols.  No tower-lsp or tokio dependency — compiles
//! to WASM for browser-based editor integration.

pub mod completion;
pub mod definitions;
pub mod diagnostics;
pub mod document;
pub mod hover;
pub mod symbols;
pub mod types;
