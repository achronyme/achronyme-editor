//! Convert parser diagnostics to LSP-compatible diagnostics.
//!
//! The parser uses 1-based line/col; output uses 0-based. Point spans
//! (where start == end) are extended to the end of the line for
//! visibility in the editor.

use crate::types::{DiagnosticSeverity, LspDiagnostic, Position, Range};

/// Convert parser diagnostics to LSP-compatible diagnostics.
pub fn map_diagnostics(errors: &[achronyme_parser::Diagnostic], text: &str) -> Vec<LspDiagnostic> {
    errors
        .iter()
        .map(|e| {
            let span = &e.primary_span;
            let start_line = span.line_start.saturating_sub(1) as u32;
            let start_col = span.col_start.saturating_sub(1) as u32;
            let end_line = span.line_end.saturating_sub(1) as u32;
            let end_col = if span.col_end > span.col_start || span.line_end > span.line_start {
                span.col_end.saturating_sub(1) as u32
            } else {
                // Point span — extend to end of line for visibility
                text.lines()
                    .nth(start_line as usize)
                    .map(|l| l.len() as u32)
                    .unwrap_or(start_col + 1)
            };

            let severity = match e.severity {
                achronyme_parser::Severity::Error => DiagnosticSeverity::Error,
                achronyme_parser::Severity::Warning => DiagnosticSeverity::Warning,
                achronyme_parser::Severity::Note => DiagnosticSeverity::Information,
                achronyme_parser::Severity::Help => DiagnosticSeverity::Hint,
            };

            LspDiagnostic {
                range: Range::new(
                    Position::new(start_line, start_col),
                    Position::new(end_line, end_col),
                ),
                severity,
                code: e.code.clone(),
                source: "ach".into(),
                message: e.message.clone(),
            }
        })
        .collect()
}
