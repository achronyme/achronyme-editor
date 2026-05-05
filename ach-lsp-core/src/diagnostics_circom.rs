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
//! - **Lowering / instantiation errors** (E200-E211): only when the
//!   source is self-contained — a `component main = X(...)` is
//!   present and no `include` statement appears. Library files (no
//!   main) and multi-file projects (includes) need a workspace-aware
//!   compile and are exercised through the server's `/api/compile`
//!   endpoint, not through the LSP.
//!
//! The in-memory `circom::compile_to_prove_ir` path skips include
//! resolution entirely, so we never trigger it on multi-file projects;
//! the alternative would be 50+ "include not found" false positives
//! every time the user types a circomlib import.

use circom::analysis::constraint_check;
use circom::compile_to_prove_ir;
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

    // Lowering pass — only on self-contained sources. The check is
    // intentionally string-based on the raw source rather than walking
    // the AST: the AST might be incomplete after a parse error, and
    // the worst case of a string false positive is "we skip lowering
    // for one keystroke", which is harmless. False negatives — running
    // lowering when we shouldn't — produce noise and regret.
    if should_run_lowering(source) {
        out.extend(run_lowering(source));
    }

    out
}

/// Conservative gate on whether to attempt lowering.
///
/// We only lower a source that:
/// 1. Declares a `component main` — without one, lowering trivially
///    fails with "no main component" on every keystroke in a library
///    file, which is noise.
/// 2. Has no `include` statements — the in-memory compile path doesn't
///    resolve includes, so a multi-file project would produce dozens
///    of "include not found" false positives. Multi-file work goes
///    through `/api/compile` server-side.
fn should_run_lowering(source: &str) -> bool {
    // Skip lines that begin with `//` so a stray exploratory comment
    // (`// TODO: add component main`) doesn't trigger lowering on every
    // keystroke against an incomplete program. Block comments
    // (`/* component main */`) still slip through; that's an acceptable
    // edge case — the dominant noise source is line comments.
    let has_main = source.lines().any(|line| {
        let trimmed = line.trim_start();
        !trimmed.starts_with("//") && trimmed.contains("component main")
    });
    let has_include = source
        .lines()
        .any(|line| line.trim_start().starts_with("include "));
    has_main && !has_include
}

/// Run the in-memory circom compile pipeline and surface any errors as
/// LSP diagnostics. Parse / constraint errors will already have been
/// reported by the earlier passes; we filter those out so the user
/// doesn't see duplicate squigglies for the same span.
///
/// Wrapped in `catch_unwind` as defense-in-depth — the upstream circom
/// crate has no known user-input panics, but the LSP must survive any
/// future regression without taking the entire language server down.
/// On panic we drop diagnostics rather than crashing; the next
/// `did_change` will retry.
fn run_lowering(source: &str) -> Vec<LspDiagnostic> {
    let result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compile_to_prove_ir(source)));
    match result {
        Ok(Ok(_)) => Vec::new(),
        Ok(Err(err)) => err
            .to_diagnostics()
            .into_iter()
            .filter(|d| !is_duplicate_of_earlier_pass(d))
            .map(|d| map_diagnostic(&d, source))
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Skip diagnostics that the parser + constraint passes already
/// reported. Lowering surfaces E200-E211 plus re-emits anything below
/// it found earlier; we want only the new information.
fn is_duplicate_of_earlier_pass(d: &CircomDiagnostic) -> bool {
    match d.code.as_deref() {
        Some(code) => {
            // Parser codes (E300-E306) and constraint codes
            // (E100-E102, W101-W103) are produced by the earlier
            // passes; only let lowering codes through.
            code.starts_with("E1") || code.starts_with("W1") || code.starts_with("E3")
        }
        None => false,
    }
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
    fn lowering_skipped_on_library_file() {
        // Library files (no `component main`) must not trigger lowering —
        // would always fail with "no main component".
        let src = r#"
pragma circom 2.0.0;

template Helper(n) {
    signal input in;
    signal output out;
    out <== in * n;
}
"#;
        assert!(
            !should_run_lowering(src),
            "library files must skip lowering"
        );
        let diags = check_circom(src);
        // No lowering errors should appear — library is constraint-clean.
        for d in &diags {
            if let Some(code) = &d.code {
                assert!(
                    !code.starts_with("E2"),
                    "library file produced unexpected lowering error: {d:?}"
                );
            }
        }
    }

    #[test]
    fn lowering_skipped_when_main_is_in_a_comment() {
        // `// TODO: add component main` is exploration text, not real
        // code — must not trigger lowering on every keystroke.
        let src = r#"
pragma circom 2.0.0;
// TODO: add component main = X(8);

template Helper() {
    signal input in;
    signal output out;
    out <== in;
}
"#;
        assert!(
            !should_run_lowering(src),
            "commented-out `component main` must not trigger lowering"
        );
    }

    #[test]
    fn lowering_skipped_when_includes_present() {
        // Multi-file projects rely on workspace-aware compilation.
        // Running the in-memory pipeline would produce false-positive
        // include-not-found diagnostics.
        let src = r#"
pragma circom 2.0.0;
include "circomlib/bitify.circom";
component main = Num2Bits(8);
"#;
        assert!(
            !should_run_lowering(src),
            "sources with `include` must defer lowering to the server"
        );
    }

    #[test]
    fn lowering_runs_on_self_contained_source() {
        // Has main, no includes — lowering should run and find this
        // template clean.
        let src = r#"
pragma circom 2.0.0;

template Square() {
    signal input in;
    signal output out;
    out <== in * in;
}

component main = Square();
"#;
        assert!(
            should_run_lowering(src),
            "self-contained source must trigger lowering"
        );
        let diags = check_circom(src);
        // Clean template; no E2xx errors expected.
        for d in &diags {
            if let Some(code) = &d.code {
                assert!(
                    !code.starts_with("E2"),
                    "clean self-contained template produced lowering error: {d:?}"
                );
            }
        }
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
