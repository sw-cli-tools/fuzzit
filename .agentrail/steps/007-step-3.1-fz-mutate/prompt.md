Phase 3, Step 3.1: fz-mutate mutation operators

Implement the mutation engine crate at crates/fz-mutate/.

Deterministic mutation operators (each in its own module):
- byte_flip(input, index) -> flip a single byte
- bit_flip(input, index, bit) -> flip a single bit
- delete_token(input, sep) -> remove one token (delimited by sep)
- duplicate_token(input, sep) -> duplicate one token
- splice(input, other, pos, len) -> insert substring from other input
- nest(input, depth) -> wrap in brackets/parens repeatedly
- numeric_substitute(input) -> replace numeric strings with boundary values
- delimiter_confuse(input) -> swap quote/bracket types
- encoding_corrupt(input) -> inject invalid UTF-8 sequences

Public API:
- fn mutate(input: &[u8], rng: &mut impl RngCore) -> Vec<u8>
  Applies a random mutation from the set above.
- fn mutate_n(input: &[u8], count: usize, rng: &mut impl RngCore) -> Vec<Vec<u8>>
  Applies count mutations, returns all results.

TDD tests (use proptest where appropriate):
- byte_flip changes exactly one byte
- bit_flip changes exactly one bit
- delete_token removes exactly one token
- duplicate_token increases length
- splice inserts bytes from other
- mutate returns Vec different from input (with high probability)
- mutate_n returns count results
- All operators handle empty input gracefully
- Numeric substitute replaces numbers with boundary values
- Property: mutated output length is reasonable (not exploding)

Dependencies: fz-core, anyhow, rand (for RNG).