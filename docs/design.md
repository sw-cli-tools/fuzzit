# Design Decisions

## Decision Record

### DD-1: Workspace with many small crates, not one monolith

**Choice**: 9 crates in a Cargo workspace (fz-core, fz-manifest, fz-corpus,
fz-exec, fz-classify, fz-mutate, fz-llm, fz-artifacts, fz-cli).

**Rationale**: Each crate has a single responsibility. This allows independent
testing, clear dependency boundaries, and follows the "always split, never
consolidate" scaling rule. No crate should exceed 7 modules.

**Tradeoff**: More Cargo.toml files to maintain. Offset by workspace-level
dependency management.

### DD-2: Child process execution, never in-process

**Choice**: All target execution happens via `std::process::Command` (child
process), never via function calls or shared memory.

**Rationale**: Safety. A crashing target must not crash fuzzit. A hanging target
must not hang fuzzit. Process isolation provides natural timeout and resource
limit enforcement.

**Tradeoff**: Slower than in-process fuzzing. Acceptable for v1 since we target
CLI tools and compilers where startup time is not the bottleneck compared to
LLM latency.

### DD-3: Ollama-first LLM integration

**Choice**: Primary LLM backend is Ollama (local/LAN). Optional OpenAI-compatible
backend as future extension.

**Rationale**: No API costs, no network dependency, works offline. The fuzzing
workload is bursty (generate N seeds, then run deterministic engine for a long
time), which suits a local model well.

**Tradeoff**: LLM seed quality depends on model capability. Mitigated by using
LLM for targeted creativity only, not as the sole mutation engine.

### DD-4: Template-based harness generation, not arbitrary code generation

**Choice**: When generating Rust regression tests, use constrained templates
where the model fills in the input data and test name, not arbitrary runtime code.

**Rationale**: Safety. Prevents LLM from generating malicious or side-effectful
code. Templates are reviewable and predictable.

**Example template**:
```rust
#[test]
fn {test_name}() {{
    let input = {input_literal};
    let result = parse_input(input);
    assert!({assertion});
}}
```

### DD-5: Deterministic classification before LLM use

**Choice**: Classify all execution results deterministically first (exit code,
stderr pattern matching). Only invoke LLM for targeted tasks (seed generation,
crash minimization hints, grammar inference).

**Rationale**: Classification is fast and deterministic. LLM calls are slow and
non-deterministic. Using rules first reduces LLM calls and improves
reproducibility.

### DD-6: Draft-only artifact model

**Choice**: All outputs (regression tests, reports, promoted cases) are written
to a separate artifacts directory. Nothing is auto-committed, auto-merged, or
auto-posted.

**Rationale**: Human review gate. Fuzzing produces many false positives. Requiring
explicit promotion prevents noise from entering the codebase.

### DD-7: TOML manifests for target configuration

**Choice**: Target descriptions use TOML files with sections for [target],
[oracle], [expectations], [seeds], [strategy].

**Rationale**: TOML is human-readable, Rust-native (serde_toml), and allows
commentary. Same manifest can be version-controlled alongside the project being
fuzzed.

### DD-8: Budget-based campaign execution

**Choice**: Campaigns take a `--budget N` parameter representing the total number
of target executions across all layers. Each layer gets a proportional share.

**Rationale**: Provides predictable runtime and cost control. Users can run
quick smoke tests (budget=100) or deep campaigns (budget=10000).

**Layer budget allocation** (default):
- Layer 1 (baseline): 30% of budget
- Layer 2 (LLM seeds): 10% of budget
- Layer 3 (mutation): 40% of budget
- Layer 4 (feedback): 20% of budget

### DD-9: Deduplication by signature, not by input

**Choice**: Cases are deduplicated by (exit_code, stderr_signature_hash), not
by input content. Two different inputs that produce the same failure signature
are considered the same finding.

**Rationale**: Reduces noise. A crash from 50 slightly different inputs is one
finding, not 50.

### DD-10: Edition 2024, no warning suppression

**Choice**: Rust 2024 edition. Zero clippy warnings enforced with `-D warnings`.
No `#[allow(...)]` attributes to suppress warnings.

**Rationale**: Consistent with all other sw-cli-tools projects. Forces
idiomatic Rust and catches issues early.

## Error Handling Strategy

- Use `anyhow::Result` for application-level errors
- Use typed error enums within crates for domain-specific errors
- Never panic in library crates (fuzzit should never crash from a crashing target)
- Log errors to stderr, write structured results to artifacts

## Testing Strategy

- Unit tests in every crate for pure logic (corpus generation, mutation, classification)
- Integration tests for manifest parsing + execution pipeline
- End-to-end tests using a trivial test binary as fuzz target
- Property-based tests (proptest) for mutation operators (mutated input should
  differ from original, should be valid UTF-8 when input was, etc.)

## Concurrency Model

- Campaign execution: sequential by default (v1). Target execution is the
  bottleneck, not CPU. Future: parallel execution with configurable concurrency.
- LLM calls: sequential with timeout. One request at a time to avoid overloading
  local Ollama instance.
