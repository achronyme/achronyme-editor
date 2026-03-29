//! Conversion from `ach_lsp_core` types to `tower_lsp_server::ls_types`.

use ach_lsp_core::types as core;
use tower_lsp_server::ls_types as lsp;

pub fn position(p: core::Position) -> lsp::Position {
    lsp::Position::new(p.line, p.character)
}

pub fn range(r: core::Range) -> lsp::Range {
    lsp::Range {
        start: position(r.start),
        end: position(r.end),
    }
}

pub fn completion_item(item: core::CompletionItem) -> lsp::CompletionItem {
    let kind = match item.kind {
        core::CompletionKind::Keyword => lsp::CompletionItemKind::KEYWORD,
        core::CompletionKind::Function => lsp::CompletionItemKind::FUNCTION,
        core::CompletionKind::Method => lsp::CompletionItemKind::METHOD,
        core::CompletionKind::Constant => lsp::CompletionItemKind::CONSTANT,
        core::CompletionKind::Snippet => lsp::CompletionItemKind::SNIPPET,
    };
    let format = match item.insert_text_format {
        core::InsertTextFormat::PlainText => lsp::InsertTextFormat::PLAIN_TEXT,
        core::InsertTextFormat::Snippet => lsp::InsertTextFormat::SNIPPET,
    };
    lsp::CompletionItem {
        label: item.label,
        kind: Some(kind),
        detail: item.detail,
        insert_text: item.insert_text,
        insert_text_format: Some(format),
        ..Default::default()
    }
}

pub fn diagnostic(d: core::LspDiagnostic) -> lsp::Diagnostic {
    let severity = match d.severity {
        core::DiagnosticSeverity::Error => lsp::DiagnosticSeverity::ERROR,
        core::DiagnosticSeverity::Warning => lsp::DiagnosticSeverity::WARNING,
        core::DiagnosticSeverity::Information => lsp::DiagnosticSeverity::INFORMATION,
        core::DiagnosticSeverity::Hint => lsp::DiagnosticSeverity::HINT,
    };
    lsp::Diagnostic {
        range: range(d.range),
        severity: Some(severity),
        code: d.code.map(lsp::NumberOrString::String),
        source: Some(d.source),
        message: d.message,
        ..Default::default()
    }
}

#[allow(deprecated)]
pub fn document_symbol(s: core::DocumentSymbol) -> lsp::DocumentSymbol {
    let kind = match s.kind {
        core::SymbolKind::Function => lsp::SymbolKind::FUNCTION,
        core::SymbolKind::Class => lsp::SymbolKind::CLASS,
        core::SymbolKind::Event => lsp::SymbolKind::EVENT,
        core::SymbolKind::Module => lsp::SymbolKind::MODULE,
    };
    lsp::DocumentSymbol {
        name: s.name,
        detail: s.detail,
        kind,
        tags: None,
        deprecated: None,
        range: range(s.range),
        selection_range: range(s.selection_range),
        children: None,
    }
}

pub fn text_edit(e: core::TextEdit) -> lsp::TextEdit {
    lsp::TextEdit::new(range(e.range), e.new_text)
}
