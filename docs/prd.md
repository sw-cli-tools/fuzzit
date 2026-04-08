# Product Requirements Document (PRD)

## Project: fuzzit -- LLM-Guided Fuzz Testing Tool

## Problem Statement

Compilers, interpreters, REPLs, CLI tools, and text-input APIs are notoriously
difficult to fuzz thoroughly with traditional tools alone. Standard fuzzers
(libFuzzer, AFL++) are excellent at high-throughput byte mutation but weak at
generating domain-specific malformed text that exercises semantic edge cases.
Conversely, asking an LLM to "make weird inputs" produces interesting results
but lacks the systematic coverage and reproducibility of deterministic fuzzing.

fuzzit bridges this gap: deterministic Rust code handles execution, safety,
scoring, and reporting, while an LLM provides targeted creativity for seed
generation and crash analysis.

## Goals

1. Discover crashes, hangs, panics, and unexpected behavior in text-input tools
2. Convert every finding into a permanent regression asset (test file or corpus entry)
3. Work with local/LAN Ollama models -- no cloud API dependency required
4. Be fully reproducible: same manifest + same budget produces comparable results
5. Support human review: draft artifacts only, never auto-merge or auto-commit
6. Fit into existing TDD workflow: fuzz findings become failing tests, then passing tests

## Non-Goals (v1)

- No auto-patch or auto-PR generation
- No auto-comment on GitHub issues
- No in-process fuzzing (no shared-memory coverage guidance)
- No network/API fuzzing beyond local REST endpoints
- No GUI testing (text input only)

## Functional Requirements

### FR-1: Manifest-Driven Target Description

Users describe fuzz targets via TOML manifests specifying:
- Binary entry point and invocation mode (stdin, args, file)
- Timeout and resource limits
- Oracle rules (success/failure exit codes, stderr patterns)
- Expectations (must_not_panic, must_not_hang, must_not_segfault)
- Seed files and strategy preferences

### FR-2: Deterministic Baseline Corpus

Generate a standard edge-case corpus without LLM involvement:
- empty string, whitespace only, single characters
- huge input, repeated delimiters, escape sequences
- invalid UTF-8, null bytes, mixed newlines
- numeric boundaries, deep nesting, weird identifiers

### FR-3: Safe Target Execution

Execute targets as child processes with:
- stdin/args/file input delivery
- Wall-time timeout enforcement (kill on exceed)
- stdout/stderr capture
- No network access, no secrets in environment

### FR-4: Deterministic Failure Classification

Classify each execution result deterministically:
- panic (thread panicked in stderr)
- hang (timeout exceeded)
- segfault (signal-based exit codes)
- unexpected exit code
- success (matches oracle)
- deduplication by (exit_code, stderr_signature)

### FR-5: LLM-Generated Seeds

Query an Ollama model to generate targeted edge-case inputs:
- Compiler targets: token stream confusion, nesting overflow, escape ambiguity
- API targets: malformed JSON/YAML, encoding mixing, field omission
- REPL targets: command sequences, mode transitions, control characters
- Config parsers: duplicate keys, recursive includes, huge numerics

### FR-6: Mutation Engine

Apply deterministic mutation operators to expand coverage:
- byte flip, bit flip, token deletion, duplication, splice
- nesting growth, numeric boundary substitution
- delimiter confusion, encoding variation

### FR-7: Feedback Loop

Retain "interesting" cases and feed them back:
- New exit code, new stderr signature, longer runtime
- New stack trace pattern
- Cases that hit new classification categories

### FR-8: Artifact Generation

Produce machine-readable and human-readable outputs:
- JSON reports (machine)
- Markdown reports (human)
- Rust regression test files (#[test] functions)
- Corpus seed files
- Replay manifests (input + expected oracle)

### FR-9: Campaign Management

Run multi-layer fuzz campaigns with configurable budget:
- Layer 1: deterministic baseline
- Layer 2: LLM seeds
- Layer 3: mutation expansion
- Layer 4: feedback-driven refinement
- Aggregate report across all layers

### FR-10: Case Minimization and Promotion

- Minimize crash reproducers (binary search on input)
- Promote interesting cases to regression tests
- Deduplicate cases before promotion

## CLI Interface

```
fuzzit targets run --manifest <path>           # Single manifest execution
fuzzit targets generate --manifest <path>      # Generate seeds for a target
fuzzit campaigns start --manifest <path> ...   # Full multi-layer campaign
fuzzit campaigns report --dir <path>           # Report on past campaign
fuzzit cases minimize --case <path>            # Minimize a crash case
fuzzit cases promote --case <path>             # Promote to regression test
```

## Success Metrics

- Number of unique crashes/hangs/panics found per campaign
- Percentage of findings promoted to regression tests
- Reduction in false-positive rate (expected failures vs real bugs)
- Time to discover known bugs in test targets
- Reproducibility: re-running same manifest/budget finds same issues

## Target Users

- Developers of compilers, interpreters, and REPLs
- Maintainers of CLI tools that parse text input
- Anyone doing TDD who wants fuzz-level coverage without manual effort

## Dependencies

- Rust 2024 edition
- Ollama (local, for LLM seed generation)
- clap (CLI argument parsing)
- serde + toml (manifest parsing)
- reqwest (Ollama HTTP client)
- tempfile (sandbox directories)
