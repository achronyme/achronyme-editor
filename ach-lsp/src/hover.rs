/// Static hover documentation for Achronyme keywords and builtin functions.
///
/// Returns a Markdown string for the given word, or `None` if the word is not
/// a known keyword or builtin.
pub fn hover_for(word: &str) -> Option<&'static str> {
    match word {
        // ── Keywords ──────────────────────────────────────────────
        "let" => Some(
            "```ach\nlet name = expr\nlet mut name = expr\n```\n\
             Declare a variable. Use `mut` for mutable bindings.",
        ),
        "mut" => Some(
            "```ach\nlet mut name = expr\n```\n\
             Mark a variable as mutable, allowing reassignment.",
        ),
        "fn" => Some(
            "```ach\nfn name(params) { body }\nfn name(params) -> Type { body }\n```\n\
             Define a function. Functions are first-class values and can capture their environment as closures.",
        ),
        "if" => Some(
            "```ach\nif cond { body } else { body }\n```\n\
             Conditional expression. In circuits, compiles to `mux` (both branches are evaluated).",
        ),
        "else" => Some(
            "```ach\nif cond { body } else { body }\n```\n\
             Else branch of a conditional.",
        ),
        "while" => Some(
            "```ach\nwhile cond { body }\n```\n\
             Loop while condition is true. Not available in circuit mode.",
        ),
        "for" => Some(
            "```ach\nfor item in list { body }\nfor i in 0..n { body }\n```\n\
             Iterate over a list or range. In circuits, loops are statically unrolled.",
        ),
        "in" => Some(
            "```ach\nfor item in collection { body }\n```\n\
             Used with `for` to iterate over a collection or range.",
        ),
        "return" => Some(
            "```ach\nreturn expr\n```\n\
             Return a value from the current function.",
        ),
        "break" => Some(
            "```ach\nbreak\n```\n\
             Exit the innermost loop. Not available in circuit mode.",
        ),
        "continue" => Some(
            "```ach\ncontinue\n```\n\
             Skip to the next iteration of the innermost loop. Not available in circuit mode.",
        ),
        "forever" => Some(
            "```ach\nforever { body }\n```\n\
             Infinite loop. Use `break` to exit. Not available in circuit mode.",
        ),
        "public" => Some(
            "```ach\npublic name\n```\n\
             Declare a public input (instance variable) in a circuit.",
        ),
        "witness" => Some(
            "```ach\nwitness name\nwitness arr[n]\n```\n\
             Declare a private input (witness variable) in a circuit.",
        ),
        "prove" => Some(
            "```ach\nlet p = prove { body }\n```\n\
             Compile a circuit, capture witness values from the enclosing scope, and generate a ZK proof inline.",
        ),
        "import" => Some(
            "```ach\nimport \"./module.ach\" as mod\n```\n\
             Import a module. Exposes exported functions and constants via the alias namespace (e.g., `mod.func()`).",
        ),
        "export" => Some(
            "```ach\nexport fn name(params) { body }\nexport let NAME = expr\n```\n\
             Mark a function or constant as public API, accessible from other modules via `import`.",
        ),
        "as" => Some(
            "```ach\nimport \"./module.ach\" as alias\n```\n\
             Assign a namespace alias to an imported module.",
        ),
        "true" => Some("`true` — Boolean literal."),
        "false" => Some("`false` — Boolean literal."),
        "nil" => Some("`nil` — The absence of a value."),

        // ── Builtins: core ────────────────────────────────────────
        "print" => Some(
            "```ach\nprint(args...)\n```\n\
             Print values to stdout, separated by spaces, followed by a newline.",
        ),
        "len" => Some(
            "```ach\nlen(value) -> Int\n```\n\
             Returns the length of a String, List, or Map.",
        ),
        "typeof" => Some(
            "```ach\ntypeof(value) -> String\n```\n\
             Returns the type name of a value (`\"Number\"`, `\"String\"`, `\"Bool\"`, `\"List\"`, `\"Map\"`, `\"Field\"`, `\"BigInt256\"`, `\"BigInt512\"`, `\"Proof\"`, `\"Function\"`, `\"Native\"`).",
        ),
        "assert" => Some(
            "```ach\nassert(condition)\n```\n\
             Assert that a condition is true. Raises `AssertionFailed` if false.",
        ),
        "time" => Some(
            "```ach\ntime() -> Int\n```\n\
             Returns the current Unix timestamp in milliseconds.",
        ),

        // ── Builtins: collections ─────────────────────────────────
        "push" => Some(
            "```ach\npush(list, item)\n```\n\
             Append an item to a list.",
        ),
        "pop" => Some(
            "```ach\npop(list) -> value\n```\n\
             Remove and return the last element from a list.",
        ),
        "keys" => Some(
            "```ach\nkeys(map) -> List\n```\n\
             Returns all keys of a map as a list of strings.",
        ),

        // ── Builtins: proof inspection ────────────────────────────
        "proof_json" => Some(
            "```ach\nproof_json(proof) -> String\n```\n\
             Extract the proof data as a JSON string.",
        ),
        "proof_public" => Some(
            "```ach\nproof_public(proof) -> String\n```\n\
             Extract the public inputs as a JSON string.",
        ),
        "proof_vkey" => Some(
            "```ach\nproof_vkey(proof) -> String\n```\n\
             Extract the verification key as a JSON string.",
        ),
        "verify_proof" => Some(
            "```ach\nverify_proof(proof) -> Bool\n```\n\
             Verify that a proof is valid.",
        ),

        // ── Builtins: strings ─────────────────────────────────────
        "substring" => Some(
            "```ach\nsubstring(str, start, end) -> String\n```\n\
             Extract a substring by character indices `[start, end)`.",
        ),
        "index_of" => Some(
            "```ach\nindex_of(str, substr) -> Int\n```\n\
             Find the first occurrence of `substr`. Returns `-1` if not found.",
        ),
        "split" => Some(
            "```ach\nsplit(str, delimiter) -> List\n```\n\
             Split a string by a delimiter into a list of strings.",
        ),
        "trim" => Some(
            "```ach\ntrim(str) -> String\n```\n\
             Remove leading and trailing whitespace.",
        ),
        "replace" => Some(
            "```ach\nreplace(str, search, replacement) -> String\n```\n\
             Replace all occurrences of `search` with `replacement`.",
        ),
        "to_upper" => Some(
            "```ach\nto_upper(str) -> String\n```\n\
             Convert a string to uppercase.",
        ),
        "to_lower" => Some(
            "```ach\nto_lower(str) -> String\n```\n\
             Convert a string to lowercase.",
        ),
        "chars" => Some(
            "```ach\nchars(str) -> List\n```\n\
             Split a string into a list of single-character strings.",
        ),

        // ── Builtins: crypto ──────────────────────────────────────
        "poseidon" => Some(
            "```ach\nposeidon(left, right) -> Field\n```\n\
             Poseidon 2-to-1 hash over BN254. Available in both VM and circuit mode (361 constraints).",
        ),
        "poseidon_many" => Some(
            "```ach\nposeidon_many(a, b, c, ...) -> Field\n```\n\
             Left-fold Poseidon hash of multiple field elements. Requires at least 2 arguments.",
        ),

        // ── Builtins: bigint ──────────────────────────────────────
        "bigint256" => Some(
            "```ach\nbigint256(value) -> BigInt256\n```\n\
             Create a 256-bit unsigned integer from an Int or String (`\"0x...\"`, `\"0b...\"`). VM only.",
        ),
        "bigint512" => Some(
            "```ach\nbigint512(value) -> BigInt512\n```\n\
             Create a 512-bit unsigned integer from an Int or String (`\"0x...\"`, `\"0b...\"`). VM only.",
        ),
        "to_bits" => Some(
            "```ach\nto_bits(bigint) -> List\n```\n\
             Convert a BigInt to a list of `0`/`1` integers in LSB-first order.",
        ),
        "from_bits" => Some(
            "```ach\nfrom_bits(bits, width) -> BigInt\n```\n\
             Create a BigInt from a list of bits. `width` must be `256` or `512`.",
        ),
        "bit_and" => Some(
            "```ach\nbit_and(a, b) -> BigInt\n```\n\
             Bitwise AND of two BigInts.",
        ),
        "bit_or" => Some(
            "```ach\nbit_or(a, b) -> BigInt\n```\n\
             Bitwise OR of two BigInts.",
        ),
        "bit_xor" => Some(
            "```ach\nbit_xor(a, b) -> BigInt\n```\n\
             Bitwise XOR of two BigInts.",
        ),
        "bit_not" => Some(
            "```ach\nbit_not(x) -> BigInt\n```\n\
             Bitwise NOT of a BigInt.",
        ),
        "bit_shl" => Some(
            "```ach\nbit_shl(x, n) -> BigInt\n```\n\
             Shift a BigInt left by `n` bits.",
        ),
        "bit_shr" => Some(
            "```ach\nbit_shr(x, n) -> BigInt\n```\n\
             Shift a BigInt right by `n` bits.",
        ),

        // ── Builtins: higher-order collections (part 1) ─────────
        "map" => Some(
            "```ach\nmap(list, fn) -> List\n```\n\
             Apply `fn` to each element and return a new list with the results.",
        ),
        "filter" => Some(
            "```ach\nfilter(list, fn) -> List\n```\n\
             Return a new list keeping only elements where `fn(element)` is true.",
        ),
        "reduce" => Some(
            "```ach\nreduce(list, init, fn) -> value\n```\n\
             Fold the list with an accumulator. `fn(acc, element)` is called for each element.",
        ),
        "for_each" => Some(
            "```ach\nfor_each(list, fn)\n```\n\
             Call `fn(element)` on each element. Returns `nil`.",
        ),
        "find" => Some(
            "```ach\nfind(list, fn) -> value | nil\n```\n\
             Return the first element where `fn(element)` is true, or `nil` if none found.",
        ),

        // ── Builtins: higher-order collections (part 2) ─────────
        "any" => Some(
            "```ach\nany(list, fn) -> Bool\n```\n\
             Return `true` if `fn(element)` is true for at least one element.",
        ),
        "all" => Some(
            "```ach\nall(list, fn) -> Bool\n```\n\
             Return `true` if `fn(element)` is true for every element.",
        ),
        "sort" => Some(
            "```ach\nsort(list, fn) -> List\n```\n\
             Return a sorted copy of the list. `fn(a, b)` should return negative, zero, or positive.",
        ),
        "flat_map" => Some(
            "```ach\nflat_map(list, fn) -> List\n```\n\
             Apply `fn` to each element (must return a list) and flatten the results one level.",
        ),
        "zip" => Some(
            "```ach\nzip(a, b) -> List\n```\n\
             Pair corresponding elements from two lists into `[a[i], b[i]]` sub-lists. Truncates to the shorter length.",
        ),

        // ── Builtins: GC introspection ─────────────────────────
        "gc_stats" => Some(
            "```ach\ngc_stats() -> Map\n```\n\
             Returns a map with GC statistics: `collections`, `total_freed`, and `heap_size`.",
        ),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keywords_have_hover() {
        for kw in [
            "let", "mut", "fn", "if", "else", "while", "for", "in", "return", "break", "continue",
            "forever", "public", "witness", "prove", "true", "false", "nil", "import", "export",
            "as",
        ] {
            assert!(hover_for(kw).is_some(), "missing hover for keyword `{kw}`");
        }
    }

    #[test]
    fn builtins_have_hover() {
        for name in [
            "print",
            "len",
            "typeof",
            "assert",
            "time",
            "push",
            "pop",
            "keys",
            "proof_json",
            "proof_public",
            "proof_vkey",
            "verify_proof",
            "substring",
            "index_of",
            "split",
            "trim",
            "replace",
            "to_upper",
            "to_lower",
            "chars",
            "poseidon",
            "poseidon_many",
            "bigint256",
            "bigint512",
            "to_bits",
            "from_bits",
            "bit_and",
            "bit_or",
            "bit_xor",
            "bit_not",
            "bit_shl",
            "bit_shr",
            "gc_stats",
            "map",
            "filter",
            "reduce",
            "for_each",
            "find",
            "any",
            "all",
            "sort",
            "flat_map",
            "zip",
        ] {
            assert!(
                hover_for(name).is_some(),
                "missing hover for builtin `{name}`"
            );
        }
    }

    #[test]
    fn unknown_returns_none() {
        assert!(hover_for("my_variable").is_none());
        assert!(hover_for("foobar").is_none());
    }

    #[test]
    fn hovers_contain_code_block() {
        // Every keyword/builtin hover should have a code block (except simple literals)
        for word in ["let", "fn", "poseidon", "len", "bit_and"] {
            let doc = hover_for(word).unwrap();
            assert!(
                doc.contains("```"),
                "hover for `{word}` should contain a code block"
            );
        }
    }
}
