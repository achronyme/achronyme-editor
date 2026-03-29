//! Go-to-definition: resolve a name at a cursor position to its definition site.
//!
//! Walks the AST to build a scope-aware definition map, then resolves
//! the identifier under the cursor. Understands the VM → prove/circuit
//! scope boundary: outer functions are visible inside prove blocks.

use achronyme_parser::ast::*;
use crate::types::{Position, Range, TextEdit};

/// A definition site: where a name was introduced.
#[derive(Clone, Debug)]
struct Def {
    /// The span of the defining name (not the whole statement).
    span: Span,
    /// Scope depth at which this name was defined (for future scope-aware resolution).
    #[allow(dead_code)]
    depth: u32,
}

/// Collected definitions, scoped by depth.
struct DefCollector {
    /// All definitions found, keyed by name. Multiple entries per name
    /// are possible (shadowing); we keep them all and resolve by proximity.
    defs: Vec<(String, Def)>,
    /// Current scope depth (0 = top-level).
    depth: u32,
}

impl DefCollector {
    fn new() -> Self {
        Self {
            defs: Vec::new(),
            depth: 0,
        }
    }

    fn push_scope(&mut self) {
        self.depth += 1;
    }

    fn pop_scope(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }

    fn add(&mut self, name: String, span: Span) {
        self.defs.push((
            name,
            Def {
                span,
                depth: self.depth,
            },
        ));
    }

    /// Collect definitions from a program.
    fn collect_program(&mut self, program: &Program) {
        for stmt in &program.stmts {
            self.collect_stmt(stmt);
        }
    }

    fn collect_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FnDecl {
                name,
                params,
                body,
                span,
                ..
            } => {
                // The function name is defined at the fn statement span.
                // Use a name-only span (approximate: start of statement to end of name).
                self.add(name.clone(), name_span_from(span, name));
                self.push_scope();
                for p in params {
                    self.add(p.name.clone(), span.clone());
                }
                self.collect_block(body);
                self.pop_scope();
            }
            Stmt::LetDecl {
                name, value, span, ..
            }
            | Stmt::MutDecl {
                name, value, span, ..
            } => {
                self.add(name.clone(), name_span_from(span, name));
                self.collect_expr(value);
            }
            Stmt::CircuitDecl {
                name,
                params,
                body,
                span,
            } => {
                self.add(name.clone(), name_span_from(span, name));
                self.push_scope();
                for p in params {
                    self.add(p.name.clone(), span.clone());
                }
                self.collect_block(body);
                self.pop_scope();
            }
            Stmt::Import { alias, span, .. } => {
                self.add(alias.clone(), name_span_from(span, alias));
            }
            Stmt::ImportCircuit { alias, span, .. } => {
                self.add(alias.clone(), name_span_from(span, alias));
            }
            Stmt::SelectiveImport { names, span, .. } => {
                for n in names {
                    self.add(n.clone(), name_span_from(span, n));
                }
            }
            Stmt::Export { inner, .. } => self.collect_stmt(inner),
            Stmt::Expr(expr) => self.collect_expr(expr),
            Stmt::Print { value, .. }
            | Stmt::Return {
                value: Some(value), ..
            } => {
                self.collect_expr(value);
            }
            Stmt::Assignment { value, .. } => {
                self.collect_expr(value);
            }
            _ => {}
        }
    }

    fn collect_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Prove {
                params, body, span, ..
            } => {
                self.push_scope();
                for p in params {
                    self.add(p.name.clone(), span.clone());
                }
                self.collect_block(body);
                self.pop_scope();
            }
            Expr::FnExpr {
                name,
                params,
                body,
                span,
                ..
            } => {
                if let Some(n) = name {
                    self.add(n.clone(), name_span_from(span, n));
                }
                self.push_scope();
                for p in params {
                    self.add(p.name.clone(), span.clone());
                }
                self.collect_block(body);
                self.pop_scope();
            }
            Expr::For {
                var, body, span, ..
            } => {
                self.push_scope();
                self.add(var.clone(), name_span_from(span, var));
                self.collect_block(body);
                self.pop_scope();
            }
            Expr::If {
                then_block,
                else_branch,
                ..
            } => {
                self.collect_block(then_block);
                if let Some(eb) = else_branch {
                    match eb {
                        ElseBranch::Block(b) => self.collect_block(b),
                        ElseBranch::If(e) => self.collect_expr(e),
                    }
                }
            }
            Expr::While { body, .. } | Expr::Forever { body, .. } => {
                self.collect_block(body);
            }
            Expr::Block(b) => self.collect_block(b),
            Expr::Call { callee, args, .. } => {
                self.collect_expr(callee);
                for arg in args {
                    self.collect_expr(&arg.value);
                }
            }
            _ => {}
        }
    }

    fn collect_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.collect_stmt(stmt);
        }
    }

    /// Find the best definition for `name` that is visible at `byte_offset`.
    /// Prefers definitions that:
    /// 1. Come before the cursor position
    /// 2. Are at the same or lower (outer) scope depth
    /// 3. Among equal candidates, prefer the closest one
    fn resolve(&self, name: &str, byte_offset: usize) -> Option<&Def> {
        self.defs
            .iter()
            .filter(|(n, def)| n == name && def.span.byte_start <= byte_offset)
            .map(|(_, def)| def)
            .last() // last definition before cursor = closest
    }
}

/// Create a span for just the name portion of a statement.
/// Approximation: uses the statement span's start line/col.
fn name_span_from(stmt_span: &Span, _name: &str) -> Span {
    stmt_span.clone()
}

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

/// Resolve the definition of the word at the given byte offset.
/// Returns the LSP Range of the definition site, or None if not found.
pub fn goto_definition(source: &str, byte_offset: usize) -> Option<Range> {
    let (program, _errors) = achronyme_parser::parse_program(source);

    // Find the word at the byte offset
    let name = word_at_byte(source, byte_offset)?;

    let mut collector = DefCollector::new();
    collector.collect_program(&program);

    let def = collector.resolve(&name, byte_offset)?;
    Some(span_to_range(&def.span))
}

/// Extract the identifier word at a byte offset in the source.
fn word_at_byte(source: &str, offset: usize) -> Option<String> {
    if offset >= source.len() {
        return None;
    }
    let bytes = source.as_bytes();
    if !is_ident_char(bytes[offset]) {
        return None;
    }
    let mut start = offset;
    while start > 0 && is_ident_char(bytes[start - 1]) {
        start -= 1;
    }
    let mut end = offset;
    while end < bytes.len() && is_ident_char(bytes[end]) {
        end += 1;
    }
    Some(source[start..end].to_string())
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Convert line/col (0-based) to byte offset in source.
pub fn position_to_byte_offset(source: &str, line: u32, col: u32) -> Option<usize> {
    let mut current_line = 0u32;
    let mut offset = 0usize;
    for l in source.lines() {
        if current_line == line {
            let col = col as usize;
            if col <= l.len() {
                return Some(offset + col);
            }
            return Some(offset + l.len());
        }
        offset += l.len() + 1; // +1 for newline
        current_line += 1;
    }
    // Cursor on last line (no trailing newline)
    if current_line == line {
        Some(offset)
    } else {
        None
    }
}

// ---------------------------------------------------------------------------
// Find all references
// ---------------------------------------------------------------------------

/// Find all occurrences of the name at `byte_offset` in the source.
/// Returns a list of LSP Ranges (includes definition + all usages).
pub fn find_references(source: &str, byte_offset: usize) -> Vec<Range> {
    let name = match word_at_byte(source, byte_offset) {
        Some(n) => n,
        None => return vec![],
    };

    let (program, _errors) = achronyme_parser::parse_program(source);

    // Collect definition sites
    let mut collector = DefCollector::new();
    collector.collect_program(&program);

    // Collect all reference sites (Ident nodes + definition spans)
    let mut spans: Vec<&Span> = Vec::new();

    // Add definition sites
    for (n, def) in &collector.defs {
        if n == &name {
            spans.push(&def.span);
        }
    }

    // Walk AST for Ident references
    let mut ref_spans: Vec<Span> = Vec::new();
    collect_ident_refs(&program, &name, &mut ref_spans);

    let mut ranges: Vec<Range> = spans.iter().map(|s| span_to_range(s)).collect();
    ranges.extend(ref_spans.iter().map(|s| span_to_range(s)));

    // Deduplicate by range start position
    ranges.sort_by_key(|r| (r.start.line, r.start.character));
    ranges.dedup_by(|a, b| a.start == b.start);

    ranges
}

/// Walk the AST collecting spans of Ident expressions matching `name`.
fn collect_ident_refs(program: &Program, name: &str, refs: &mut Vec<Span>) {
    for stmt in &program.stmts {
        collect_stmt_refs(stmt, name, refs);
    }
}

fn collect_stmt_refs(stmt: &Stmt, name: &str, refs: &mut Vec<Span>) {
    match stmt {
        Stmt::FnDecl { body, .. } => collect_block_refs(body, name, refs),
        Stmt::LetDecl { value, .. } | Stmt::MutDecl { value, .. } => {
            collect_expr_refs(value, name, refs);
        }
        Stmt::Assignment { target, value, .. } => {
            collect_expr_refs(target, name, refs);
            collect_expr_refs(value, name, refs);
        }
        Stmt::CircuitDecl { body, .. } => collect_block_refs(body, name, refs),
        Stmt::Export { inner, .. } => collect_stmt_refs(inner, name, refs),
        Stmt::Expr(expr) => collect_expr_refs(expr, name, refs),
        Stmt::Print { value, .. }
        | Stmt::Return {
            value: Some(value), ..
        } => {
            collect_expr_refs(value, name, refs);
        }
        _ => {}
    }
}

fn collect_expr_refs(expr: &Expr, name: &str, refs: &mut Vec<Span>) {
    match expr {
        Expr::Ident { name: n, span } if n == name => {
            refs.push(span.clone());
        }
        Expr::Call { callee, args, .. } => {
            collect_expr_refs(callee, name, refs);
            for arg in args {
                collect_expr_refs(&arg.value, name, refs);
            }
        }
        Expr::BinOp { lhs, rhs, .. } => {
            collect_expr_refs(lhs, name, refs);
            collect_expr_refs(rhs, name, refs);
        }
        Expr::UnaryOp { operand, .. } => collect_expr_refs(operand, name, refs),
        Expr::Index { object, index, .. } => {
            collect_expr_refs(object, name, refs);
            collect_expr_refs(index, name, refs);
        }
        Expr::DotAccess { object, .. } => collect_expr_refs(object, name, refs),
        Expr::If {
            condition,
            then_block,
            else_branch,
            ..
        } => {
            collect_expr_refs(condition, name, refs);
            collect_block_refs(then_block, name, refs);
            if let Some(eb) = else_branch {
                match eb {
                    ElseBranch::Block(b) => collect_block_refs(b, name, refs),
                    ElseBranch::If(e) => collect_expr_refs(e, name, refs),
                }
            }
        }
        Expr::For { body, .. } => collect_block_refs(body, name, refs),
        Expr::While {
            condition, body, ..
        } => {
            collect_expr_refs(condition, name, refs);
            collect_block_refs(body, name, refs);
        }
        Expr::Forever { body, .. } => collect_block_refs(body, name, refs),
        Expr::Block(b) => collect_block_refs(b, name, refs),
        Expr::Prove { body, .. } => collect_block_refs(body, name, refs),
        Expr::FnExpr { body, .. } => collect_block_refs(body, name, refs),
        Expr::Array { elements, .. } => {
            for e in elements {
                collect_expr_refs(e, name, refs);
            }
        }
        _ => {}
    }
}

fn collect_block_refs(block: &Block, name: &str, refs: &mut Vec<Span>) {
    for stmt in &block.stmts {
        collect_stmt_refs(stmt, name, refs);
    }
}

// ---------------------------------------------------------------------------
// Rename
// ---------------------------------------------------------------------------

/// Check if the word at `byte_offset` is renameable. Returns the range and
/// current name if so. Builtins and keywords are not renameable.
pub fn prepare_rename(source: &str, byte_offset: usize) -> Option<(Range, String)> {
    let name = word_at_byte(source, byte_offset)?;

    // Don't rename builtins or keywords
    if crate::hover::hover_for(&name).is_some() {
        return None;
    }

    // Must have at least one definition
    let (program, _) = achronyme_parser::parse_program(source);
    let mut collector = DefCollector::new();
    collector.collect_program(&program);

    if collector.resolve(&name, source.len()).is_none() {
        return None;
    }

    // Return the range of the word under cursor
    let start = byte_offset
        - source.as_bytes()[..byte_offset]
            .iter()
            .rev()
            .take_while(|b| is_ident_char(**b))
            .count();
    let end = byte_offset
        + source.as_bytes()[byte_offset..]
            .iter()
            .take_while(|b| is_ident_char(**b))
            .count();

    let line_start = source[..start].matches('\n').count() as u32;
    let col_start = (start - source[..start].rfind('\n').map(|p| p + 1).unwrap_or(0)) as u32;
    let line_end = source[..end].matches('\n').count() as u32;
    let col_end = (end - source[..end].rfind('\n').map(|p| p + 1).unwrap_or(0)) as u32;

    Some((
        Range::new(
            Position::new(line_start, col_start),
            Position::new(line_end, col_end),
        ),
        name,
    ))
}

/// Rename all occurrences of the name at `byte_offset` to `new_name`.
/// Returns a list of TextEdit for building a WorkspaceEdit.
pub fn rename(source: &str, byte_offset: usize, new_name: &str) -> Vec<TextEdit> {
    find_references(source, byte_offset)
        .into_iter()
        .map(|range| TextEdit {
            range,
            new_text: new_name.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_def(source: &str, target: &str) -> Option<Range> {
        // Find the LAST occurrence of target (simulating cursor on usage, not definition)
        let offset = source.rfind(target)?;
        goto_definition(source, offset)
    }

    #[test]
    fn goto_fn_definition() {
        let source = "fn double(x) { x * 2 }\nlet y = double(5)";
        let range = find_def(source, "double").unwrap();
        // Definition is on line 0
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn goto_let_definition() {
        let source = "let x = 42\nlet y = x + 1";
        let range = find_def(source, "x").unwrap();
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn goto_circuit_definition() {
        let source = "circuit vote(r: Public) { assert_eq(r, r) }\nlet p = vote(r: 5)";
        let range = find_def(source, "vote").unwrap();
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn goto_import_definition() {
        let source = "import \"./utils.ach\" as utils\nlet x = utils.helper()";
        let range = find_def(source, "utils").unwrap();
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn goto_fn_param() {
        let source = "fn add(a, b) { a + b }";
        // Put cursor on the `a` in `a + b` (usage inside body)
        let offset = source.rfind("a + b").unwrap(); // points to 'a' in body
        let range = goto_definition(source, offset).unwrap();
        // Should resolve to the param definition on line 0
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn outer_fn_visible_in_prove() {
        let source = "fn helper(x) { x * 2 }\nprove(out: Public) { assert_eq(helper(3), out) }";
        // Cursor on `helper` inside prove block
        let prove_helper = source.rfind("helper").unwrap();
        let range = goto_definition(source, prove_helper).unwrap();
        // Should resolve to fn definition on line 0
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn unknown_name_returns_none() {
        let source = "let x = 42";
        assert!(goto_definition(source, source.find("42").unwrap()).is_none());
    }

    #[test]
    fn for_loop_var() {
        let source = "for i in 0..10 { print(i) }";
        let usage = source.rfind('i').unwrap();
        let range = goto_definition(source, usage).unwrap();
        assert_eq!(range.start.line, 0);
    }

    #[test]
    fn shadowing_prefers_closest() {
        let source = "let x = 1\nlet x = 2\nprint(x)";
        let usage = source.rfind('x').unwrap();
        let range = goto_definition(source, usage).unwrap();
        // Should prefer the second `let x` (line 1)
        assert_eq!(range.start.line, 1);
    }

    #[test]
    fn position_to_offset() {
        let source = "line0\nline1\nline2";
        assert_eq!(position_to_byte_offset(source, 0, 0), Some(0));
        assert_eq!(position_to_byte_offset(source, 1, 0), Some(6));
        assert_eq!(position_to_byte_offset(source, 2, 3), Some(15));
    }

    // ── Find all references ───────────────────────────────────

    #[test]
    fn refs_fn_definition_and_calls() {
        let source = "fn double(x) { x * 2 }\nlet a = double(3)\nlet b = double(5)";
        let offset = source.find("double").unwrap();
        let refs = find_references(source, offset);
        // definition + 2 calls = 3
        assert_eq!(refs.len(), 3);
    }

    #[test]
    fn refs_let_binding_and_usages() {
        let source = "let x = 42\nlet y = x + 1\nprint(x)";
        let offset = source.find('x').unwrap();
        let refs = find_references(source, offset);
        // definition + 2 usages = 3
        assert_eq!(refs.len(), 3);
    }

    #[test]
    fn refs_across_prove_boundary() {
        let source = "fn helper(x) { x * 2 }\nprove(out: Public) { assert_eq(helper(3), out) }";
        let offset = source.find("helper").unwrap();
        let refs = find_references(source, offset);
        // definition + usage inside prove = 2
        assert_eq!(refs.len(), 2);
    }

    #[test]
    fn refs_unknown_name_empty() {
        let source = "let x = 42";
        let offset = source.find("42").unwrap();
        let refs = find_references(source, offset);
        assert!(refs.is_empty());
    }

    // ── Rename ────────────────────────────────────────────────

    #[test]
    fn rename_fn_all_occurrences() {
        let source = "fn double(x) { x * 2 }\nlet a = double(3)";
        let offset = source.find("double").unwrap();
        let edits = rename(source, offset, "triple");
        // definition + 1 call = 2
        assert_eq!(edits.len(), 2);
        assert!(edits.iter().all(|e| e.new_text == "triple"));
    }

    #[test]
    fn prepare_rename_rejects_builtins() {
        let source = "poseidon(a, b)";
        let offset = source.find("poseidon").unwrap();
        assert!(prepare_rename(source, offset).is_none());
    }

    #[test]
    fn prepare_rename_accepts_user_fn() {
        let source = "fn double(x) { x * 2 }\ndouble(5)";
        let offset = source.rfind("double").unwrap();
        let (_, name) = prepare_rename(source, offset).unwrap();
        assert_eq!(name, "double");
    }
}
