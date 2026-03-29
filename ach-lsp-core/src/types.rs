//! Core types for the Achronyme LSP — no tower-lsp dependency.
//!
//! These mirror LSP protocol types but are standalone so they can
//! compile to WASM and be serialized to JSON via serde.

use serde::Serialize;

/// A position in a text document (0-based line and character).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Self { line, character }
    }
}

/// A range in a text document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// A completion item.
#[derive(Clone, Debug, Serialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub insert_text: Option<String>,
    pub insert_text_format: InsertTextFormat,
}

/// Kind of a completion item.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum CompletionKind {
    Keyword,
    Function,
    Method,
    Constant,
    Snippet,
}

/// Format of insert text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
}

/// A diagnostic message.
#[derive(Clone, Debug, Serialize)]
pub struct LspDiagnostic {
    pub range: Range,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
    pub source: String,
    pub message: String,
}

/// Severity of a diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// A symbol in a document (function, circuit, prove block, import).
#[derive(Clone, Debug, Serialize)]
pub struct DocumentSymbol {
    pub name: String,
    pub detail: Option<String>,
    pub kind: SymbolKind,
    pub range: Range,
    pub selection_range: Range,
}

/// Kind of a document symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum SymbolKind {
    Function,
    Class,
    Event,
    Module,
}

/// A text edit (for rename operations).
#[derive(Clone, Debug, Serialize)]
pub struct TextEdit {
    pub range: Range,
    pub new_text: String,
}
