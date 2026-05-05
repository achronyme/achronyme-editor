use crate::types::*;

/// Build completion items for all Achronyme keywords.
pub fn keyword_completions() -> Vec<CompletionItem> {
    [
        "let", "mut", "fn", "if", "else", "while", "for", "in", "return", "break", "continue",
        "forever", "public", "witness", "prove", "circuit", "true", "false", "nil", "print",
        "import", "export", "as", "Public", "Witness", "Field", "Bool",
    ]
    .into_iter()
    .map(|kw| CompletionItem {
        label: kw.to_string(),
        kind: CompletionKind::Keyword,
        detail: None,
        insert_text: None,
        insert_text_format: InsertTextFormat::PlainText,
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

/// Build completion items for `.circom` keywords. Independent from
/// [`keyword_completions`] because the keyword sets only partially
/// overlap (`signal`, `template`, `component`, `pragma` are circom-only;
/// `let`, `mut`, `fn`, `circuit` are achronyme-only).
pub fn circom_keyword_completions() -> Vec<CompletionItem> {
    [
        "pragma",
        "include",
        "template",
        "component",
        "signal",
        "input",
        "output",
        "function",
        "var",
        "for",
        "while",
        "if",
        "else",
        "return",
        "main",
        "public",
    ]
    .into_iter()
    .map(|kw| CompletionItem {
        label: kw.to_string(),
        kind: CompletionKind::Keyword,
        detail: None,
        insert_text: None,
        insert_text_format: InsertTextFormat::PlainText,
    })
    .collect()
}

/// Snippet completions tailored for `.circom` source — template
/// scaffolding, signal declarations, constraint operators, and the
/// circomlib templates the achronyme front-end has been verified
/// against.
pub fn circom_snippet_completions() -> Vec<CompletionItem> {
    let snippets: &[(&str, &str, &str, CompletionKind)] = &[
        (
            "pragma",
            "pragma circom 2.0.0;\n",
            "Required version directive at the top of every .circom file",
            CompletionKind::Snippet,
        ),
        (
            "template",
            "template ${1:Name}(${2:n}) {\n\tsignal input ${3:in};\n\tsignal output ${4:out};\n\t$0\n}",
            "Define a parametric template",
            CompletionKind::Snippet,
        ),
        (
            "component",
            "component main = ${1:Name}(${2:8});",
            "Instantiate the main component",
            CompletionKind::Snippet,
        ),
        (
            "signal input",
            "signal input ${1:name};",
            "Declare an input signal",
            CompletionKind::Snippet,
        ),
        (
            "signal output",
            "signal output ${1:name};",
            "Declare an output signal",
            CompletionKind::Snippet,
        ),
        (
            "include",
            "include \"${1:bitify.circom}\";",
            "Include another .circom file",
            CompletionKind::Snippet,
        ),
        (
            "function",
            "function ${1:nbits}(${2:a}) {\n\tvar n = 1;\n\twhile (n < ${2:a}) { n *= 2; }\n\treturn n;\n}",
            "Compile-time helper function",
            CompletionKind::Snippet,
        ),
        (
            "for",
            "for (var ${1:i} = 0; ${1:i} < ${2:n}; ${1:i}++) {\n\t$0\n}",
            "C-style for loop (compile-time unrolled)",
            CompletionKind::Snippet,
        ),
        // Constraint operators — included as snippets so users discover the
        // three forms. Hover docs explain when to use each.
        (
            "<==",
            "${1:lhs} <== ${2:rhs};",
            "Constrain + assign (preferred): emits both `<--` and `===`",
            CompletionKind::Snippet,
        ),
        (
            "<--",
            "${1:lhs} <-- ${2:hint_expr};",
            "Witness hint only — assigns off-circuit, no constraint",
            CompletionKind::Snippet,
        ),
        (
            "===",
            "${1:lhs} === ${2:rhs};",
            "Constraint only — must already be assigned",
            CompletionKind::Snippet,
        ),
        // Circomlib component instantiations — round-trip-verified through
        // the achronyme circom front-end.
        (
            "Num2Bits",
            "component ${1:n2b} = Num2Bits(${2:8});",
            "Decompose a field element into n little-endian bits",
            CompletionKind::Function,
        ),
        (
            "LessThan",
            "component ${1:lt} = LessThan(${2:8});",
            "Boolean: in[0] < in[1] (n-bit inputs)",
            CompletionKind::Function,
        ),
        (
            "IsZero",
            "component ${1:iz} = IsZero();",
            "Boolean: in == 0",
            CompletionKind::Function,
        ),
        (
            "Poseidon",
            "component ${1:hash} = Poseidon(${2:2});",
            "ZK-friendly hash over BN254 (~240 constraints for Poseidon(2))",
            CompletionKind::Function,
        ),
        (
            "MiMCSponge",
            "component ${1:hash} = MiMCSponge(${2:2}, 220, 1);",
            "MiMC sponge hash (~3,087 constraints for the 2→1 form)",
            CompletionKind::Function,
        ),
        (
            "Pedersen",
            "component ${1:hash} = Pedersen(${2:n});",
            "Pedersen hash on Baby Jubjub — outputs a curve point",
            CompletionKind::Function,
        ),
        (
            "Sha256",
            "component ${1:hash} = Sha256(${2:64});",
            "SHA-256 — heavy (~30k constraints for SHA-256(64))",
            CompletionKind::Function,
        ),
    ];

    snippets
        .iter()
        .map(|(label, insert, detail, kind)| CompletionItem {
            label: label.to_string(),
            kind: *kind,
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: InsertTextFormat::Snippet,
        })
        .collect()
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
            kind: CompletionKind::Function,
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: InsertTextFormat::Snippet,
        })
        .collect()
}

/// Type methods (47 unique names, deduplicated across Int, String, List, Map, Field, BigInt).
/// These are invoked via `.method()` syntax.
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
            ".to_string() -> String — Convert to String (Int, String, Field, BigInt, Bool, List)",
        ),
        (
            "to_hex",
            ".to_hex() -> String — BigInt hex representation (0x-prefixed)",
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
            kind: CompletionKind::Method,
            detail: Some(detail.to_string()),
            insert_text: Some(label.to_string()),
            insert_text_format: InsertTextFormat::PlainText,
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
            kind: CompletionKind::Constant,
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: InsertTextFormat::Snippet,
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
        (
            "prove",
            "prove(${1:name}: Public) {\n\t$0\n}",
            "Prove block with typed params",
        ),
        (
            "proven",
            "prove ${1:name}(${2:input}: Public) {\n\t$0\n}",
            "Named prove block",
        ),
        (
            "circuit",
            "circuit ${1:name}(${2:input}: Public, ${3:secret}: Witness) {\n\t$0\n}",
            "Circuit definition",
        ),
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
            kind: CompletionKind::Snippet,
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: InsertTextFormat::Snippet,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_count() {
        assert_eq!(keyword_completions().len(), 27);
    }

    #[test]
    fn keywords_are_keyword_kind() {
        for item in keyword_completions() {
            assert_eq!(item.kind, CompletionKind::Keyword);
        }
    }

    #[test]
    fn builtin_count() {
        let items = builtin_completions();
        assert_eq!(items.len(), 16);
        for item in &items {
            assert_eq!(item.kind, CompletionKind::Function);
        }
    }

    #[test]
    fn method_count() {
        let items = method_completions();
        assert_eq!(items.len(), 47);
        for item in &items {
            assert_eq!(item.kind, CompletionKind::Method);
        }
    }

    #[test]
    fn static_count() {
        let items = static_completions();
        assert_eq!(items.len(), 6);
        for item in &items {
            assert_eq!(item.kind, CompletionKind::Constant);
        }
    }

    #[test]
    fn snippet_count() {
        let items = snippet_completions();
        let snippets: Vec<_> = items
            .iter()
            .filter(|i| i.kind == CompletionKind::Snippet)
            .collect();
        assert_eq!(snippets.len(), 12);
    }

    #[test]
    fn builtins_have_snippet_format() {
        let items = builtin_completions();
        for item in &items {
            assert_eq!(
                item.insert_text_format,
                InsertTextFormat::Snippet,
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
                InsertTextFormat::PlainText,
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
                InsertTextFormat::Snippet,
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
        for item in items.iter().filter(|i| i.kind == CompletionKind::Snippet) {
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
        // 16 globals + 47 methods + 6 statics + 12 code snippets = 81
        assert_eq!(items.len(), 81);
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

    #[test]
    fn circom_keyword_completions_present() {
        let items = circom_keyword_completions();
        let labels: std::collections::HashSet<_> = items.iter().map(|i| i.label.clone()).collect();
        for kw in [
            "pragma",
            "include",
            "template",
            "component",
            "signal",
            "input",
            "output",
            "function",
            "var",
            "main",
        ] {
            assert!(
                labels.contains(kw),
                "circom keyword completions missing `{kw}`"
            );
        }
        for item in &items {
            assert_eq!(item.kind, CompletionKind::Keyword);
        }
    }

    #[test]
    fn circom_snippets_cover_circomlib() {
        let items = circom_snippet_completions();
        let labels: std::collections::HashSet<_> = items.iter().map(|i| i.label.clone()).collect();
        for tpl in ["Num2Bits", "LessThan", "IsZero", "Poseidon", "Sha256"] {
            assert!(
                labels.contains(tpl),
                "circom snippet completions missing `{tpl}`"
            );
        }
        // Constraint operators discoverable via completion
        for op in ["<==", "<--", "==="] {
            assert!(
                labels.contains(op),
                "circom snippet completions missing operator `{op}`"
            );
        }
    }

    #[test]
    fn circom_completions_are_disjoint_from_ach() {
        // `circuit`, `let`, `mut`, `import`, `export` are .ach-only.
        let circom_labels: std::collections::HashSet<_> = circom_keyword_completions()
            .iter()
            .map(|i| i.label.clone())
            .collect();
        for ach_only in ["let", "mut", "circuit", "import", "export", "prove"] {
            assert!(
                !circom_labels.contains(ach_only),
                "circom keyword completions leaked .ach-only token `{ach_only}`"
            );
        }
    }
}
