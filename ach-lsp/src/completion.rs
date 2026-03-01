use tower_lsp_server::ls_types::*;

/// Build completion items for all Achronyme keywords.
pub fn keyword_completions() -> Vec<CompletionItem> {
    [
        "let", "mut", "fn", "if", "else", "while", "for", "in", "return", "break", "continue",
        "forever", "public", "witness", "prove", "true", "false", "nil", "print",
    ]
    .into_iter()
    .map(|kw| CompletionItem {
        label: kw.to_string(),
        kind: Some(CompletionItemKind::KEYWORD),
        ..Default::default()
    })
    .collect()
}

/// Build completion items for all Achronyme builtin functions and code snippets.
pub fn snippet_completions() -> Vec<CompletionItem> {
    let mut items = builtin_completions();
    items.extend(code_snippets());
    items
}

fn builtin_completions() -> Vec<CompletionItem> {
    let builtins: &[(&str, &str, &str)] = &[
        // (label, insert_text, detail)
        // Core
        (
            "print",
            "print($1)",
            "print(args...) — Print values to stdout",
        ),
        ("len", "len($1)", "len(value) -> Int"),
        ("typeof", "typeof($1)", "typeof(value) -> String"),
        ("assert", "assert($1)", "assert(condition)"),
        ("time", "time()", "time() -> Int — Unix timestamp (ms)"),
        // Collections
        ("push", "push($1, $2)", "push(list, item)"),
        ("pop", "pop($1)", "pop(list) -> value"),
        ("keys", "keys($1)", "keys(map) -> List"),
        // Proof inspection
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
            "verify_proof",
            "verify_proof($1)",
            "verify_proof(proof) -> Bool",
        ),
        // Strings
        (
            "substring",
            "substring($1, $2, $3)",
            "substring(str, start, end) -> String",
        ),
        ("indexOf", "indexOf($1, $2)", "indexOf(str, substr) -> Int"),
        ("split", "split($1, $2)", "split(str, delimiter) -> List"),
        ("trim", "trim($1)", "trim(str) -> String"),
        (
            "replace",
            "replace($1, $2, $3)",
            "replace(str, search, replacement) -> String",
        ),
        ("toUpper", "toUpper($1)", "toUpper(str) -> String"),
        ("toLower", "toLower($1)", "toLower(str) -> String"),
        ("chars", "chars($1)", "chars(str) -> List"),
        // Crypto
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
        // BigInt
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
        ("to_bits", "to_bits($1)", "to_bits(bigint) -> List"),
        (
            "from_bits",
            "from_bits($1, $2)",
            "from_bits(bits, width) -> BigInt",
        ),
        ("bit_and", "bit_and($1, $2)", "bit_and(a, b) -> BigInt"),
        ("bit_or", "bit_or($1, $2)", "bit_or(a, b) -> BigInt"),
        ("bit_xor", "bit_xor($1, $2)", "bit_xor(a, b) -> BigInt"),
        ("bit_not", "bit_not($1)", "bit_not(x) -> BigInt"),
        ("bit_shl", "bit_shl($1, $2)", "bit_shl(x, n) -> BigInt"),
        ("bit_shr", "bit_shr($1, $2)", "bit_shr(x, n) -> BigInt"),
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
        assert_eq!(keyword_completions().len(), 19);
    }

    #[test]
    fn keywords_are_keyword_kind() {
        for item in keyword_completions() {
            assert_eq!(item.kind, Some(CompletionItemKind::KEYWORD));
        }
    }

    #[test]
    fn builtin_count() {
        let items = snippet_completions();
        let builtins: Vec<_> = items
            .iter()
            .filter(|i| i.kind == Some(CompletionItemKind::FUNCTION))
            .collect();
        assert_eq!(builtins.len(), 32);
    }

    #[test]
    fn snippet_count() {
        let items = snippet_completions();
        let snippets: Vec<_> = items
            .iter()
            .filter(|i| i.kind == Some(CompletionItemKind::SNIPPET))
            .collect();
        assert_eq!(snippets.len(), 7);
    }

    #[test]
    fn builtins_have_snippet_format() {
        let items = snippet_completions();
        for item in items
            .iter()
            .filter(|i| i.kind == Some(CompletionItemKind::FUNCTION))
        {
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
}
