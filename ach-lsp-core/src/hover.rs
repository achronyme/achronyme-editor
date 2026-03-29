/// Static hover documentation for Achronyme keywords, global functions,
/// methods, and static namespaces.
///
/// Returns a Markdown string for the given word, or `None` if the word is not
/// a known keyword, global, method, or type namespace.
pub fn hover_for(word: &str) -> Option<&'static str> {
    match word {
        // ── Keywords ──────────────────────────────────────────────
        "let" => Some(
            "```ach\nlet name = expr\n```\n\
             Declare an immutable variable. Use `mut` instead for mutable bindings.",
        ),
        "mut" => Some(
            "```ach\nmut name = expr\n```\n\
             Declare a mutable variable, allowing reassignment with `name = new_value`.",
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
             Declare a public input in a standalone circuit file.\n\n\
             In circuit/prove params: `name: Public` or `name: Public Field[N]`.",
        ),
        "witness" => Some(
            "```ach\nwitness name\nwitness arr[n]\n```\n\
             Declare a private input in a standalone circuit file.\n\n\
             In circuit params: `name: Witness` or `name: Witness Field[N]`.\n\
             In prove blocks, witnesses are auto-captured from outer scope.",
        ),
        "prove" => Some(
            "```ach\nprove(hash: Public) { body }\nprove name(root: Public) { body }\n```\n\
             Compile a circuit via ProveIR and generate a ZK proof inline.\n\n\
             Only public inputs are declared — all other referenced variables are auto-captured as witnesses.\n\
             Named proves (`prove name(...)`) desugar to `let name = prove name(...)`.",
        ),
        "circuit" => Some(
            "```ach\ncircuit name(root: Public, leaf: Witness, path: Witness Field[3]) {\n    body\n}\n```\n\
             Define a reusable circuit. Parameters require `Public` or `Witness` visibility.\n\n\
             Call with keyword args: `name(root: val, leaf: val)`.\n\
             Import from files: `import circuit \"./file.ach\" as name`.",
        ),
        "Public" => Some(
            "`Public` — Visibility modifier for circuit/prove parameters.\n\n\
             Public inputs are part of the proof statement (visible to verifiers).\n\n\
             ```ach\ncircuit hash_check(output: Public, secret: Witness) { ... }\nprove(hash: Public) { ... }\n```",
        ),
        "Witness" => Some(
            "`Witness` — Visibility modifier for circuit parameters.\n\n\
             Witness inputs are private (known only to the prover).\n\n\
             ```ach\ncircuit merkle(root: Public, path: Witness Field[3]) { ... }\n```\n\n\
             In prove blocks, witnesses are auto-captured — no `Witness` annotation needed.",
        ),
        "Bool" => Some(
            "`Bool` — Boolean type annotation.\n\n\
             ```ach\nlet flag: Bool = true\ncircuit check(flag: Public Bool) { ... }\n```",
        ),
        "import" => Some(
            "```ach\nimport \"./module.ach\" as mod\nimport { func } from \"./module.ach\"\nimport circuit \"./circuit.ach\" as name\n```\n\
             Import a module, selective names, or a circuit file.",
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

        // ── Global functions (16) ───────────────────────────────
        "print" => Some(
            "```ach\nprint(args...)\n```\n\
             Print values to stdout, separated by spaces, followed by a newline.",
        ),
        "typeof" => Some(
            "```ach\ntypeof(value) -> String\n```\n\
             Returns the type name of a value (`\"Int\"`, `\"String\"`, `\"Bool\"`, `\"List\"`, `\"Map\"`, `\"Field\"`, `\"BigInt256\"`, `\"BigInt512\"`, `\"Proof\"`, `\"Function\"`, `\"Native\"`).",
        ),
        "assert" => Some(
            "```ach\nassert(condition)\n```\n\
             Assert that a condition is true. Raises `AssertionFailed` if false.",
        ),
        "time" => Some(
            "```ach\ntime() -> Int\n```\n\
             Returns the current Unix timestamp in milliseconds.",
        ),
        "gc_stats" => Some(
            "```ach\ngc_stats() -> Map\n```\n\
             Returns a map with GC statistics: `collections`, `total_freed`, and `heap_size`.",
        ),
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
        "poseidon" => Some(
            "```ach\nposeidon(left, right) -> Field\n```\n\
             Poseidon 2-to-1 hash over BN254. Available in both VM and circuit mode (361 constraints).",
        ),
        "poseidon_many" => Some(
            "```ach\nposeidon_many(a, b, c, ...) -> Field\n```\n\
             Left-fold Poseidon hash of multiple field elements. Requires at least 2 arguments.",
        ),
        "verify_proof" => Some(
            "```ach\nverify_proof(proof) -> Bool\n```\n\
             Verify that a proof is valid.",
        ),
        "bigint256" => Some(
            "```ach\nbigint256(value) -> BigInt256\n```\n\
             Create a 256-bit unsigned integer from an Int or String (`\"0x...\"`, `\"0b...\"`). VM only.",
        ),
        "bigint512" => Some(
            "```ach\nbigint512(value) -> BigInt512\n```\n\
             Create a 512-bit unsigned integer from an Int or String (`\"0x...\"`, `\"0b...\"`). VM only.",
        ),
        "from_bits" => Some(
            "```ach\nfrom_bits(bits, width) -> BigInt\n```\n\
             Create a BigInt from a list of bits. `width` must be `256` or `512`.",
        ),
        "parse_int" => Some(
            "```ach\nparse_int(str) -> Int\n```\n\
             Parse a string as an integer.",
        ),
        "join" => Some(
            "```ach\njoin(list, separator) -> String\n```\n\
             Join a list of values into a string with the given separator.",
        ),

        // ── Methods ─────────────────────────────────────────────
        // These use `.method()` syntax on a value.

        // -- Universal / multi-type --
        "len" => Some(
            "```ach\nvalue.len() -> Int\n```\n\
             Returns the length of a String, List, or Map.",
        ),
        "to_string" => Some(
            "```ach\nvalue.to_string() -> String\n```\n\
             String representation. Available on Int, String, Field, BigInt, Bool, and List.\n\
             Lists format as `[1, 2, 3]` with recursive element formatting.",
        ),
        "to_hex" => Some(
            "```ach\nbigint.to_hex() -> String\n```\n\
             Hex string representation of a BigInt, prefixed with `0x`.",
        ),

        // -- List --
        "push" => Some(
            "```ach\nlist.push(item)\n```\n\
             Append an item to a list.",
        ),
        "pop" => Some(
            "```ach\nlist.pop() -> value\n```\n\
             Remove and return the last element from a list.",
        ),
        "map" => Some(
            "```ach\nlist.map(fn) -> List\n```\n\
             Apply `fn` to each element and return a new list with the results.",
        ),
        "filter" => Some(
            "```ach\nlist.filter(fn) -> List\n```\n\
             Return a new list keeping only elements where `fn(element)` is true.",
        ),
        "reduce" => Some(
            "```ach\nlist.reduce(init, fn) -> value\n```\n\
             Fold the list with an accumulator. `fn(acc, element)` is called for each element.",
        ),
        "for_each" => Some(
            "```ach\nlist.for_each(fn)\n```\n\
             Call `fn(element)` on each element. Returns `nil`.",
        ),
        "find" => Some(
            "```ach\nlist.find(fn) -> value | nil\n```\n\
             Return the first element where `fn(element)` is true, or `nil` if none found.",
        ),
        "any" => Some(
            "```ach\nlist.any(fn) -> Bool\n```\n\
             Return `true` if `fn(element)` is true for at least one element.",
        ),
        "all" => Some(
            "```ach\nlist.all(fn) -> Bool\n```\n\
             Return `true` if `fn(element)` is true for every element.",
        ),
        "sort" => Some(
            "```ach\nlist.sort(fn) -> List\n```\n\
             Return a sorted copy of the list. `fn(a, b)` should return negative, zero, or positive.",
        ),
        "flat_map" => Some(
            "```ach\nlist.flat_map(fn) -> List\n```\n\
             Apply `fn` to each element (must return a list) and flatten the results one level.",
        ),
        "zip" => Some(
            "```ach\nlist.zip(other) -> List\n```\n\
             Pair corresponding elements from two lists into `[a[i], b[i]]` sub-lists. Truncates to the shorter length.",
        ),

        // -- Map --
        "keys" => Some(
            "```ach\nmap.keys() -> List\n```\n\
             Returns all keys of a map as a list of strings.",
        ),
        "values" => Some(
            "```ach\nmap.values() -> List\n```\n\
             Returns all values of a map as a list.",
        ),
        "entries" => Some(
            "```ach\nmap.entries() -> List\n```\n\
             Returns a list of `[key, value]` pairs.",
        ),
        "contains_key" => Some(
            "```ach\nmap.contains_key(key) -> Bool\n```\n\
             Check if a key exists in the map.",
        ),
        "get" => Some(
            "```ach\nmap.get(key, default) -> value\n```\n\
             Get the value for `key`, returning `default` if not found.",
        ),
        "set" => Some(
            "```ach\nmap.set(key, value)\n```\n\
             Add or update an entry in the map.",
        ),
        "remove" => Some(
            "```ach\nmap.remove(key) -> value\n```\n\
             Remove and return the value for `key`.",
        ),

        // -- String --
        "substring" => Some(
            "```ach\nstr.substring(start, end) -> String\n```\n\
             Extract a substring by character indices `[start, end)`.",
        ),
        "index_of" => Some(
            "```ach\nstr.index_of(substr) -> Int\n```\n\
             Find the first occurrence of `substr`. Returns `-1` if not found.",
        ),
        "split" => Some(
            "```ach\nstr.split(delimiter) -> List\n```\n\
             Split a string by a delimiter into a list of strings.",
        ),
        "trim" => Some(
            "```ach\nstr.trim() -> String\n```\n\
             Remove leading and trailing whitespace.",
        ),
        "replace" => Some(
            "```ach\nstr.replace(search, replacement) -> String\n```\n\
             Replace all occurrences of `search` with `replacement`.",
        ),
        "to_upper" => Some(
            "```ach\nstr.to_upper() -> String\n```\n\
             Convert a string to uppercase.",
        ),
        "to_lower" => Some(
            "```ach\nstr.to_lower() -> String\n```\n\
             Convert a string to lowercase.",
        ),
        "chars" => Some(
            "```ach\nstr.chars() -> List\n```\n\
             Split a string into a list of single-character strings.",
        ),
        "starts_with" => Some(
            "```ach\nstr.starts_with(prefix) -> Bool\n```\n\
             Return `true` if the string starts with `prefix`.",
        ),
        "ends_with" => Some(
            "```ach\nstr.ends_with(suffix) -> Bool\n```\n\
             Return `true` if the string ends with `suffix`.",
        ),
        "contains" => Some(
            "```ach\nstr.contains(substr) -> Bool\n```\n\
             Return `true` if the string contains `substr`.",
        ),
        "repeat" => Some(
            "```ach\nstr.repeat(n) -> String\n```\n\
             Repeat the string `n` times.",
        ),

        // -- Int --
        "abs" => Some(
            "```ach\nint.abs() -> Int\n```\n\
             Returns the absolute value.",
        ),
        "min" => Some(
            "```ach\nint.min(other) -> Int\n```\n\
             Returns the smaller of two integers.",
        ),
        "max" => Some(
            "```ach\nint.max(other) -> Int\n```\n\
             Returns the larger of two integers.",
        ),
        "pow" => Some(
            "```ach\nint.pow(exp) -> Int\n```\n\
             Raise to the power of `exp`.",
        ),
        "to_field" => Some(
            "```ach\nint.to_field() -> Field\n```\n\
             Convert an integer to a BN254 field element.",
        ),

        // -- Field --
        "to_int" => Some(
            "```ach\nfield.to_int() -> Int\n```\n\
             Convert a field element to an integer. The value must fit in 64 bits.",
        ),

        // -- BigInt --
        "to_bits" => Some(
            "```ach\nbigint.to_bits() -> List\n```\n\
             Convert a BigInt to a list of `0`/`1` integers in LSB-first order.",
        ),
        "bit_and" => Some(
            "```ach\nbigint.bit_and(other) -> BigInt\n```\n\
             Bitwise AND of two BigInts.",
        ),
        "bit_or" => Some(
            "```ach\nbigint.bit_or(other) -> BigInt\n```\n\
             Bitwise OR of two BigInts.",
        ),
        "bit_xor" => Some(
            "```ach\nbigint.bit_xor(other) -> BigInt\n```\n\
             Bitwise XOR of two BigInts.",
        ),
        "bit_not" => Some(
            "```ach\nbigint.bit_not() -> BigInt\n```\n\
             Bitwise NOT of a BigInt.",
        ),
        "bit_shl" => Some(
            "```ach\nbigint.bit_shl(n) -> BigInt\n```\n\
             Shift a BigInt left by `n` bits.",
        ),
        "bit_shr" => Some(
            "```ach\nbigint.bit_shr(n) -> BigInt\n```\n\
             Shift a BigInt right by `n` bits.",
        ),

        // ── Static namespaces ───────────────────────────────────
        "Int" => Some(
            "```ach\nInt::MAX\nInt::MIN\n```\n\
             60-bit signed integer type.\n\n\
             - `Int::MAX` — largest representable integer\n\
             - `Int::MIN` — smallest representable integer",
        ),
        "Field" => Some(
            "```ach\nField::ZERO\nField::ONE\nField::ORDER\n```\n\
             BN254 scalar field element type.\n\n\
             - `Field::ZERO` — additive identity\n\
             - `Field::ONE` — multiplicative identity\n\
             - `Field::ORDER` — prime modulus of BN254 Fr",
        ),
        "BigInt" => Some(
            "```ach\nBigInt::from_bits(bits, width) -> BigInt\n```\n\
             BigInt type namespace.\n\n\
             - `BigInt::from_bits(bits, width)` — construct a BigInt from an LSB-first bit list. `width` must be `256` or `512`.",
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
            "forever", "public", "witness", "prove", "circuit", "true", "false", "nil", "import",
            "export", "as", "Public", "Witness", "Bool",
        ] {
            assert!(hover_for(kw).is_some(), "missing hover for keyword `{kw}`");
        }
    }

    #[test]
    fn builtins_have_hover() {
        // Global functions (16)
        for name in [
            "print",
            "typeof",
            "assert",
            "time",
            "gc_stats",
            "proof_json",
            "proof_public",
            "proof_vkey",
            "poseidon",
            "poseidon_many",
            "verify_proof",
            "bigint256",
            "bigint512",
            "from_bits",
            "parse_int",
            "join",
        ] {
            assert!(
                hover_for(name).is_some(),
                "missing hover for global function `{name}`"
            );
        }

        // Methods (45)
        for name in [
            "len",
            "push",
            "pop",
            "keys",
            "values",
            "entries",
            "contains_key",
            "get",
            "set",
            "remove",
            "substring",
            "index_of",
            "split",
            "trim",
            "replace",
            "to_upper",
            "to_lower",
            "chars",
            "starts_with",
            "ends_with",
            "contains",
            "repeat",
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
            "abs",
            "min",
            "max",
            "pow",
            "to_field",
            "to_string",
            "to_hex",
            "to_int",
            "to_bits",
            "bit_and",
            "bit_or",
            "bit_xor",
            "bit_not",
            "bit_shl",
            "bit_shr",
        ] {
            assert!(
                hover_for(name).is_some(),
                "missing hover for method `{name}`"
            );
        }

        // Static namespaces (3)
        for name in ["Int", "Field", "BigInt"] {
            assert!(
                hover_for(name).is_some(),
                "missing hover for static namespace `{name}`"
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
        // Every keyword/global/method hover should have a code block (except simple literals)
        for word in ["let", "fn", "poseidon", "len", "bit_and", "Int", "Field"] {
            let doc = hover_for(word).unwrap();
            assert!(
                doc.contains("```"),
                "hover for `{word}` should contain a code block"
            );
        }
    }
}
