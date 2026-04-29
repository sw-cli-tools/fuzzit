Phase 1, Step 1.4: fz-corpus baseline edge corpus

Implement the baseline corpus generation crate at crates/fz-corpus/.

Generate deterministic edge-case inputs without any LLM involvement.

Generators to implement (each as a separate function in its own module):
- empty_inputs(): Vec<Vec<u8>> -- empty string
- whitespace_inputs(): Vec<Vec<u8>> -- space, tab, newline, mixed
- single_char_inputs(): Vec<Vec<u8>> -- each ASCII printable char
- huge_input(size: usize): Vec<u8> -- repeated 'A' up to size
- delimiter_inputs(): Vec<Vec<u8>> -- repeated brackets, braces, parens, quotes
- escape_inputs(): Vec<Vec<u8>> -- backslash sequences, unicode escapes
- invalid_utf8_inputs(): Vec<Vec<u8>> -- various invalid byte sequences
- null_byte_inputs(): Vec<Vec<u8>> -- null bytes in various positions
- newline_variants(): Vec<Vec<u8>> -- CRLF, CR, LF, mixed
- numeric_boundary_inputs(): Vec<Vec<u8>> -- 0, -1, i64::MAX, i64::MIN, f64 special values as strings
- deep_nesting(depth: usize): Vec<Vec<u8>> -- nested brackets/parens
- weird_identifiers(): Vec<Vec<u8>> -- reserved words, unicode, very long, empty-like

Public API:
- fn generate_baseline_corpus() -> Vec<CaseInput>
  where CaseInput { data: Vec<u8>, description: String }

TDD: write failing tests for each generator verifying:
- Output is non-empty
- Specific expected values are present (e.g., empty_inputs contains empty vec)
- No generator panics
- Description is non-empty and meaningful

Dependencies: fz-core (for CaseInput type if shared, or define locally).