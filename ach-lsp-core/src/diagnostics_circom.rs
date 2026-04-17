//! Diagnostics for `.circom` source files.
//!
//! Wraps the `circom` crate's parser + constraint analyzer behind the
//! same `LspDiagnostic` shape as [`crate::diagnostics`] so the LSP
//! backend can publish both file types through one pipeline.
//!
//! Covered checks:
//!
//! - **Parser errors** (E300-E306 and uncoded parse failures).
//! - **Constraint soundness** (E100-E102): `<--` without matching
//!   `===`, under-constrained signals.
//! - **Constraint warnings** (W101-W103): unreferenced input/output
//!   signals, weaker unverified assignments.
//!
//! Lowering / instantiation errors (E200-E211) are not produced here
//! — those require a main component and full pipeline, which an
//! editor open on a single file shouldn't trigger.

use circom::analysis::constraint_check;
use circom::parser;
use diagnostics::{Diagnostic as CircomDiagnostic, Severity as CircomSeverity, SpanRange};

use crate::types::{DiagnosticSeverity, LspDiagnostic, Position, Range};

/// Parse `.circom` source and return LSP-compatible diagnostics.
///
/// This is the primary entry point for `.circom` files. It:
/// 1. Runs the parser, surfacing parse errors.
/// 2. If parsing produces a `CircomProgram`, runs constraint analysis
///    and surfaces its errors + warnings (E100-E102 / W101-W103).
pub fn check_circom(source: &str) -> Vec<LspDiagnostic> {
    // Parse — this always returns a (CircomProgram, Vec<Diagnostic>) tuple
    // when parsing can recover; only lexer-level disasters return Err.
    let (program, parse_diags) = match parser::parse_circom(source) {
        Ok(r) => r,
        Err(e) => {
            // Lexer / unrecoverable parse: synthesize a single diagnostic
            // at the start of the file so the editor has something to show.
            return vec![LspDiagnostic {
                range: Range::new(Position::new(0, 0), Position::new(0, 0)),
                severity: DiagnosticSeverity::Error,
                code: None,
                source: "circom".into(),
                message: e.to_string(),
            }];
        }
    };

    let mut out: Vec<LspDiagnostic> = parse_diags
        .iter()
        .map(|d| map_diagnostic(d, source))
        .collect();

    // Constraint analysis reports are per-template; flatten.
    for report in constraint_check::check_constraints(&program.definitions) {
        for d in &report.diagnostics {
            out.push(map_diagnostic(d, source));
        }
    }

    out
}

/// Convert a circom-side `Diagnostic` (shared `diagnostics` crate) into
/// the LSP shape. Mirrors the `.ach` path in [`crate::diagnostics`]:
/// 1-based line/col → 0-based, point spans extended to end of line.
fn map_diagnostic(d: &CircomDiagnostic, text: &str) -> LspDiagnostic {
    let span: &SpanRange = &d.primary_span;
    let start_line = span.line_start.saturating_sub(1) as u32;
    let start_col = span.col_start.saturating_sub(1) as u32;
    let end_line = span.line_end.saturating_sub(1) as u32;
    let end_col = if span.col_end > span.col_start || span.line_end > span.line_start {
        span.col_end.saturating_sub(1) as u32
    } else {
        // Point span — extend to end of line for visibility.
        text.lines()
            .nth(start_line as usize)
            .map(|l| l.len() as u32)
            .unwrap_or(start_col + 1)
    };

    let severity = match d.severity {
        CircomSeverity::Error => DiagnosticSeverity::Error,
        CircomSeverity::Warning => DiagnosticSeverity::Warning,
        CircomSeverity::Note => DiagnosticSeverity::Information,
        CircomSeverity::Help => DiagnosticSeverity::Hint,
    };

    LspDiagnostic {
        range: Range::new(
            Position::new(start_line, start_col),
            Position::new(end_line, end_col),
        ),
        severity,
        code: d.code.clone(),
        source: "circom".into(),
        message: d.message.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_template_produces_no_diagnostics() {
        let src = r#"
pragma circom 2.0.0;

template Pair() {
    signal input a;
    signal input b;
    signal output c;
    c <== a + b;
}

component main = Pair();
"#;
        let diags = check_circom(src);
        assert!(
            diags.is_empty(),
            "expected no diagnostics, got {}: {:?}",
            diags.len(),
            diags
        );
    }

    #[test]
    fn under_constrained_signal_flagged_as_e100() {
        // `out <-- in * 2` without a `=== in * 2` is the canonical E100.
        let src = r#"
pragma circom 2.0.0;

template Broken() {
    signal input in;
    signal output out;
    out <-- in * 2;
}

component main = Broken();
"#;
        let diags = check_circom(src);
        assert!(
            !diags.is_empty(),
            "expected at least one diagnostic for under-constrained signal"
        );
        // E100 lives in the constraint analyzer.
        let has_error = diags
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error);
        assert!(
            has_error,
            "expected an error-severity diagnostic: {diags:?}"
        );
    }

    #[test]
    fn parse_error_surfaced_as_diagnostic() {
        // Hard parse error: missing semicolon + nonsense token.
        let src = "pragma circom @@@";
        let diags = check_circom(src);
        assert!(!diags.is_empty(), "expected a parse diagnostic");
        // Parse failures always carry "circom" source.
        assert!(
            diags.iter().all(|d| d.source == "circom"),
            "expected source=circom on parse diagnostics"
        );
    }

    #[test]
    fn lsp_positions_are_zero_based() {
        // Feed a template with an E100 and verify the span is converted.
        let src = "pragma circom 2.0.0;\ntemplate Bad() {\n    signal output out;\n    out <-- 42;\n}\ncomponent main = Bad();\n";
        let diags = check_circom(src);
        let error = diags
            .iter()
            .find(|d| d.severity == DiagnosticSeverity::Error)
            .expect("expected an E100");
        // 1-based line 4 in source → 0-based line 3 in LSP.
        assert_eq!(error.range.start.line, 3);
        // Columns are 0-based; anything reasonable inside the line is fine.
        assert!(error.range.start.character < error.range.end.character);
    }
}
