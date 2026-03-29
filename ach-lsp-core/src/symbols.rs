//! Document symbol extraction for the VS Code outline panel.
//!
//! Walks the AST produced by `parse_program` and extracts top-level
//! declarations: functions, circuit definitions, and named prove blocks.

use achronyme_parser::ast::{Expr, Span, Stmt};
use crate::types::*;

/// Convert a parser Span (1-based) to an LSP Range (0-based).
fn span_to_range(span: &Span) -> Range {
    Range::new(
        Position::new(
            span.line_start.saturating_sub(1) as u32,
            span.col_start.saturating_sub(1) as u32,
        ),
        Position::new(
            span.line_end.saturating_sub(1) as u32,
            span.col_end.saturating_sub(1) as u32,
        ),
    )
}

/// Extract document symbols from a parsed program.
pub fn document_symbols(source: &str) -> Vec<DocumentSymbol> {
    let (program, _errors) = achronyme_parser::parse_program(source);
    let mut symbols = Vec::new();

    for stmt in &program.stmts {
        collect_stmt_symbols(stmt, &mut symbols);
    }

    symbols
}

fn collect_stmt_symbols(stmt: &Stmt, symbols: &mut Vec<DocumentSymbol>) {
    match stmt {
        Stmt::FnDecl {
            name,
            params,
            return_type,
            span,
            ..
        } => {
            let detail = format!(
                "fn({}){}",
                params
                    .iter()
                    .map(|p| p.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                return_type
                    .as_ref()
                    .map(|t| format!(" -> {}", t.base))
                    .unwrap_or_default()
            );
            let range = span_to_range(span);
            symbols.push(DocumentSymbol {
                name: name.clone(),
                detail: Some(detail),
                kind: SymbolKind::Function,
                range,
                selection_range: range,
            });
        }
        Stmt::CircuitDecl {
            name, params, span, ..
        } => {
            let detail = format!(
                "circuit({})",
                params
                    .iter()
                    .map(|p| {
                        let vis = p
                            .type_ann
                            .as_ref()
                            .and_then(|ta| ta.visibility)
                            .map(|v| match v {
                                achronyme_parser::ast::Visibility::Public => "Public",
                                achronyme_parser::ast::Visibility::Witness => "Witness",
                            })
                            .unwrap_or("?");
                        format!("{}: {vis}", p.name)
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let range = span_to_range(span);
            symbols.push(DocumentSymbol {
                name: name.clone(),
                detail: Some(detail),
                kind: SymbolKind::Class,
                range,
                selection_range: range,
            });
        }
        Stmt::LetDecl { value, .. } | Stmt::MutDecl { value, .. } => {
            // Check if the value is a named prove block
            if let Expr::Prove {
                name: Some(prove_name),
                params,
                span: prove_span,
                ..
            } = value
            {
                let detail = format!(
                    "prove({})",
                    params
                        .iter()
                        .map(|p| p.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let range = span_to_range(prove_span);
                symbols.push(DocumentSymbol {
                    name: prove_name.clone(),
                    detail: Some(detail),
                    kind: SymbolKind::Event,
                    range,
                    selection_range: range,
                });
            }
        }
        Stmt::Expr(expr) => {
            // Top-level prove expression (named)
            if let Expr::Prove {
                name: Some(prove_name),
                params,
                span,
                ..
            } = expr
            {
                let detail = format!(
                    "prove({})",
                    params
                        .iter()
                        .map(|p| p.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                let range = span_to_range(span);
                symbols.push(DocumentSymbol {
                    name: prove_name.clone(),
                    detail: Some(detail),
                    kind: SymbolKind::Event,
                    range,
                    selection_range: range,
                });
            }
        }
        Stmt::Import { alias, span, .. } => {
            let range = span_to_range(span);
            symbols.push(DocumentSymbol {
                name: alias.clone(),
                detail: Some("import".into()),
                kind: SymbolKind::Module,
                range,
                selection_range: range,
            });
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_functions() {
        let symbols = document_symbols("fn double(x) { x * 2 }\nfn add(a, b) { a + b }");
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].name, "double");
        assert_eq!(symbols[0].kind, SymbolKind::Function);
        assert_eq!(symbols[0].detail.as_deref(), Some("fn(x)"));
        assert_eq!(symbols[1].name, "add");
        assert_eq!(symbols[1].detail.as_deref(), Some("fn(a, b)"));
    }

    #[test]
    fn extracts_circuits() {
        let symbols = document_symbols(
            "circuit merkle(root: Public, leaf: Witness) { assert_eq(root, leaf) }",
        );
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "merkle");
        assert_eq!(symbols[0].kind, SymbolKind::Class);
        assert_eq!(
            symbols[0].detail.as_deref(),
            Some("circuit(root: Public, leaf: Witness)")
        );
    }

    #[test]
    fn extracts_named_prove() {
        let symbols = document_symbols("prove membership(root: Public) { assert_eq(root, root) }");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "membership");
        assert_eq!(symbols[0].kind, SymbolKind::Event);
        assert_eq!(symbols[0].detail.as_deref(), Some("prove(root)"));
    }

    #[test]
    fn extracts_imports() {
        let symbols = document_symbols("import \"./utils.ach\" as utils");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "utils");
        assert_eq!(symbols[0].kind, SymbolKind::Module);
    }

    #[test]
    fn mixed_file() {
        let source = r#"
import "./hash.ach" as h

fn helper(x) { x * 2 }

circuit vote(root: Public, secret: Witness) {
    assert_eq(root, secret)
}

prove membership(root: Public) {
    assert_eq(root, root)
}
"#;
        let symbols = document_symbols(source);
        let names: Vec<&str> = symbols.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["h", "helper", "vote", "membership"]);
    }

    #[test]
    fn empty_file() {
        let symbols = document_symbols("");
        assert!(symbols.is_empty());
    }
}
