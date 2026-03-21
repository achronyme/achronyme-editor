use tower_lsp_server::ls_types::*;

/// Build completion items for all Achronyme keywords.
pub fn keyword_completions() -> Vec<CompletionItem> {
    [
        "let", "mut", "fn", "if", "else", "while", "for", "in", "return", "break", "continue",
        "forever", "public", "witness", "prove", "true", "false", "nil", "print", "import",
        "export", "as",
    ]
    .into_iter()
    .map(|kw| CompletionItem {
        label: kw.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    })
    .collect()
}

/// Build completion items for all Achronyme builtin functions, methods, statics, and code snippets.
pub fn snippet_completions() -> Vec<CompletionItem> {
    let mut items = builtin_completions();
    items.extend(method_completions());
    items.extend(static_completions());
    items.extend(code_snippets());
    items
}

/// Global functions (16 items). These are the only functions remaining at global scope in beta.13.
fn builtin_completions() -> Vec<CompletionItem> {
    let builtins: &[(&str, &str, &str)] = &[
        // (label, insert_text, detail)
        (
            "print",
            "print($1)",
            "print(args...) — Print values to stdout",
        ),
        ("typeof", "typeof($1)", "typeof(value) -> String"),
        ("assert", "assert($1)", "assert(condition)"),
        ("time", "time()", "time() -> Int — Unix timestamp (ms)"),
        (
            "gc_stats",
            "gc_stats()",
            "gc_stats() -> Map — GC statistics",
        ),
        (
            "proof_json",
            "proof_json($1)",
            "proof_json(proof) -> String",
        ),
        (
            "proof_public",
            "proof_public($1)",
            "proof_public(proof) -> String",
        ),
        (
            "proof_vkey",
            "proof_vkey($1)",
            "proof_vkey(proof) -> String",
        ),
        (
            "poseidon",
            "poseidon($1, $2)",
            "poseidon(left, right) -> Field",
        ),
        (
            "poseidon_many",
            "poseidon_many($1, $2)",
            "poseidon_many(a, b, ...) -> Field",
        ),
        (
            "verify_proof",
            "verify_proof($1)",
            "verify_proof(proof) -> Bool",
        ),
        (
            "bigint256",
            "bigint256($1)",
            "bigint256(value) -> BigInt256",
        ),
        (
            "bigint512",
            "bigint512($1)",
            "bigint512(value) -> BigInt512",
        ),
        (
            "from_bits",
            "from_bits($1, $2)",
            "from_bits(bits, width) -> BigInt",
        ),
        ("parse_int", "parse_int($1)", "parse_int(str) -> Int"),
        ("join", "join($1, $2)", "join(list, separator) -> String"),
    ];

    builtins
        .iter()
        .map(|(label, insert, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

/// Type methods (46 unique names, deduplicated across Int, String, List, Map, Field, BigInt).
/// These are invoked via `.method()` syntax in beta.13.
fn method_completions() -> Vec<CompletionItem> {
    let methods: &[(&str, &str)] = &[
        // (label, detail)
        // Int methods
        ("abs", ".abs() -> Int — Absolute value of Int"),
        ("min", ".min(other) -> Int — Minimum of two Ints"),
        ("max", ".max(other) -> Int — Maximum of two Ints"),
        ("pow", ".pow(exp) -> Int — Raise Int to power"),
        (
            "to_field",
            ".to_field() -> Field — Convert Int to Field element",
        ),
        (
            "to_string",
            ".to_string() -> String — Convert to String (Int, String, Field)",
        ),
        // String methods (excluding to_string, already listed)
        ("len", ".len() -> Int — Length of String, List, or Map"),
        (
            "starts_with",
            ".starts_with(prefix) -> Bool — Check String prefix",
        ),
        (
            "ends_with",
            ".ends_with(suffix) -> Bool — Check String suffix",
        ),
        (
            "contains",
            ".contains(substr) -> Bool — Check substring in String",
        ),
        ("split", ".split(delim) -> List — Split String by delimiter"),
        ("trim", ".trim() -> String — Trim whitespace from String"),
        (
            "replace",
            ".replace(search, repl) -> String — Replace in String",
        ),
        (
            "to_upper",
            ".to_upper() -> String — Convert String to uppercase",
        ),
        (
            "to_lower",
            ".to_lower() -> String — Convert String to lowercase",
        ),
        (
            "chars",
            ".chars() -> List — Split String into character list",
        ),
        (
            "index_of",
            ".index_of(substr) -> Int — Find substring position in String",
        ),
        (
            "substring",
            ".substring(start, end) -> String — Extract substring",
        ),
        ("repeat", ".repeat(n) -> String — Repeat String n times"),
        // List methods (excluding len, already listed)
        ("push", ".push(item) — Append item to List"),
        ("pop", ".pop() -> value — Remove last item from List"),
        ("map", ".map(fn) -> List — Apply fn to each List element"),
        (
            "filter",
            ".filter(fn) -> List — Keep List elements where fn returns true",
        ),
        (
            "reduce",
            ".reduce(init, fn) -> value — Fold List with accumulator",
        ),
        (
            "for_each",
            ".for_each(fn) — Call fn on each List element (side effects)",
        ),
        (
            "find",
            ".find(fn) -> value | nil — First List element where fn returns true",
        ),
        (
            "any",
            ".any(fn) -> Bool — True if fn returns true for any List element",
        ),
        (
            "all",
            ".all(fn) -> Bool — True if fn returns true for all List elements",
        ),
        (
            "sort",
            ".sort(fn) -> List — Sort List by comparator fn(a, b) -> Int",
        ),
        (
            "flat_map",
            ".flat_map(fn) -> List — Map then flatten one level",
        ),
        ("zip", ".zip(other) -> List — Pair elements from two Lists"),
        // Map methods (excluding len, already listed)
        ("keys", ".keys() -> List — Get all keys from Map"),
        ("values", ".values() -> List — Get all values from Map"),
        (
            "entries",
            ".entries() -> List — Get [key, value] pairs from Map",
        ),
        (
            "contains_key",
            ".contains_key(key) -> Bool — Check if Map contains key",
        ),
        (
            "get",
            ".get(key, default) -> value — Get Map value with default",
        ),
        ("set", ".set(key, value) — Set key-value pair in Map"),
        ("remove", ".remove(key) -> value — Remove key from Map"),
        // Field methods (excluding to_string, already listed)
        ("to_int", ".to_int() -> Int — Convert Field element to Int"),
        // BigInt methods
        ("to_bits", ".to_bits() -> List — Convert BigInt to bit list"),
        ("bit_and", ".bit_and(other) -> BigInt — Bitwise AND"),
        ("bit_or", ".bit_or(other) -> BigInt — Bitwise OR"),
        ("bit_xor", ".bit_xor(other) -> BigInt — Bitwise XOR"),
        ("bit_not", ".bit_not() -> BigInt — Bitwise NOT"),
        ("bit_shl", ".bit_shl(n) -> BigInt — Bitwise shift left"),
        ("bit_shr", ".bit_shr(n) -> BigInt — Bitwise shift right"),
    ];

    methods
        .iter()
        .map(|(label, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::METHOD),
            detail: Some(detail.to_string()),
            insert_text: Some(label.to_string()),
            insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
            ..Default::default()
        })
        .collect()
}

/// Static namespace members (6 items). Accessed via `Type::MEMBER` syntax.
fn static_completions() -> Vec<CompletionItem> {
    let statics: &[(&str, &str, &str)] = &[
        // (label, insert_text, detail)
        (
            "Int::MAX",
            "Int::MAX",
            "Int::MAX — Maximum 60-bit signed integer (2^59 - 1)",
        ),
        (
            "Int::MIN",
            "Int::MIN",
            "Int::MIN — Minimum 60-bit signed integer (-2^59)",
        ),
        (
            "Field::ZERO",
            "Field::ZERO",
            "Field::ZERO — Field element 0",
        ),
        ("Field::ONE", "Field::ONE", "Field::ONE — Field element 1"),
        (
            "Field::ORDER",
            "Field::ORDER",
            "Field::ORDER — BN254 Fr modulus (string)",
        ),
        (
            "BigInt::from_bits",
            "BigInt::from_bits($1, $2)",
            "BigInt::from_bits(bits, width) -> BigInt",
        ),
    ];

    statics
        .iter()
        .map(|(label, insert, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::CONSTANT),
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

fn code_snippets() -> Vec<CompletionItem> {
    let snippets: &[(&str, &str, &str)] = &[
        (
            "fn",
            "fn ${1:name}(${2:params}) {\n\t$0\n}",
            "Define a function",
        ),
        ("prove", "prove {\n\t$0\n}", "Prove block"),
        (
            "for",
            "for ${1:item} in ${2:collection} {\n\t$0\n}",
            "For-in loop",
        ),
        (
            "forr",
            "for ${1:i} in ${2:0}..${3:n} {\n\t$0\n}",
            "For-range loop",
        ),
        ("if", "if ${1:condition} {\n\t$0\n}", "If block"),
        (
            "ife",
            "if ${1:condition} {\n\t$2\n} else {\n\t$0\n}",
            "If-else block",
        ),
        ("while", "while ${1:condition} {\n\t$0\n}", "While loop"),
        (
            "import",
            "import \"${1:./module.ach}\" as ${2:name}",
            "Import a module",
        ),
        (
            "export fn",
            "export fn ${1:name}(${2:params}) {\n\t$0\n}",
            "Export a function",
        ),
        (
            "export let",
            "export let ${1:name} = ${0:expr}",
            "Export a constant",
        ),
    ];

    snippets
        .iter()
        .map(|(label, insert, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_count() {
        assert_eq!(keyword_completions().len(), 22);
    }

    #[test]
    fn keywords_are_keyword_kind() {
        for item in keyword_completions() {
            assert_eq!(item.kind, Some(CompletionItemKind::KEYWORD));
        }
    }

    #[test]
    fn builtin_count() {
        let items = builtin_completions();
        assert_eq!(items.len(), 16);
        for item in &items {
            assert_eq!(item.kind, Some(CompletionItemKind::FUNCTION));
        }
    }

    #[test]
    fn method_count() {
        let items = method_completions();
        assert_eq!(items.len(), 46);
        for item in &items {
            assert_eq!(item.kind, Some(CompletionItemKind::METHOD));
        }
    }

    #[test]
    fn static_count() {
        let items = static_completions();
        assert_eq!(items.len(), 6);
        for item in &items {
            assert_eq!(item.kind, Some(CompletionItemKind::CONSTANT));
        }
    }

    #[test]
    fn snippet_count() {
        let items = snippet_completions();
        let snippets: Vec<_> = items
            .iter()
            .filter(|i| i.kind == Some(CompletionItemKind::SNIPPET))
            .collect();
        assert_eq!(snippets.len(), 10);
    }

    #[test]
    fn builtins_have_snippet_format() {
        let items = builtin_completions();
        for item in &items {
            assert_eq!(
                item.insert_text_format,
                Some(InsertTextFormat::SNIPPET),
                "builtin `{}` should use snippet format",
                item.label
            );
            assert!(
                item.detail.is_some(),
                "builtin `{}` should have a detail",
                item.label
            );
        }
    }

    #[test]
    fn methods_have_plain_text_format() {
        let items = method_completions();
        for item in &items {
            assert_eq!(
                item.insert_text_format,
                Some(InsertTextFormat::PLAIN_TEXT),
                "method `{}` should use plain text format",
                item.label
            );
            assert!(
                item.detail.is_some(),
                "method `{}` should have a detail",
                item.label
            );
        }
    }

    #[test]
    fn statics_have_snippet_format() {
        let items = static_completions();
        for item in &items {
            assert_eq!(
                item.insert_text_format,
                Some(InsertTextFormat::SNIPPET),
                "static `{}` should use snippet format",
                item.label
            );
            assert!(
                item.detail.is_some(),
                "static `{}` should have a detail",
                item.label
            );
        }
    }

    #[test]
    fn snippets_contain_tabstops() {
        let items = snippet_completions();
        for item in items
            .iter()
            .filter(|i| i.kind == Some(CompletionItemKind::SNIPPET))
        {
            let text = item.insert_text.as_ref().unwrap();
            assert!(
                text.contains("$0") || text.contains("$1") || text.contains("${1:"),
                "snippet `{}` should contain tab-stops",
                item.label
            );
        }
    }

    #[test]
    fn snippet_completions_total_count() {
        let items = snippet_completions();
        // 16 globals + 46 methods + 6 statics + 10 code snippets = 78
        assert_eq!(items.len(), 78);
    }

    #[test]
    fn method_names_are_unique() {
        let items = method_completions();
        let mut seen = std::collections::HashSet::new();
        for item in &items {
            assert!(
                seen.insert(item.label.clone()),
                "duplicate method name: {}",
                item.label
            );
        }
    }
}
