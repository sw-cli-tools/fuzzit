# Implementation Plan

## Overview

Build fuzzit incrementally in phases. Each phase produces a working, testable
tool that provides value on its own. Later phases extend earlier ones.

## Phase 1: Foundation

**Goal**: Workspace setup, core types, manifest parsing, baseline corpus.

### Step 1.1: Workspace scaffold

- Convert Cargo.toml to workspace root
- Create all 9 crate directories with Cargo.toml stubs
- Set up workspace-level dependencies
- Verify `cargo build`, `cargo test`, `cargo clippy` pass
- Create a trivial test binary for end-to-end testing

### Step 1.2: fz-core -- shared types

- Define core types: FuzzTarget, ExecutionResult, CaseRecord, CampaignConfig
- Define error types: FuzzitError enum
- Unit tests for type construction and validation
- Document all public types

### Step 1.3: fz-manifest -- TOML parsing

- Parse target manifests from TOML files
- Validate required fields, provide useful errors for missing/invalid fields
- Round-trip test: parse -> serialize -> parse produces same result
- Support all sections: [target], [oracle], [expectations], [seeds], [strategy]

### Step 1.4: fz-corpus -- baseline edge corpus

- Generate deterministic edge-case inputs
- Empty, whitespace, single chars, huge input, delimiters, escapes
- Invalid UTF-8, null bytes, mixed newlines
- Numeric boundaries, deep nesting, weird identifiers
- Unit tests: each generator produces expected output, no panics

## Phase 2: Execution and Classification

**Goal**: Run targets safely, classify results, produce first reports.

### Step 2.1: fz-exec -- sandboxed target execution

- Spawn child process with stdin/args/file input
- Enforce wall-time timeout (kill on exceed)
- Capture stdout, stderr, exit code
- Resource limits (rlimit where supported)
- Integration tests using trivial test binary

### Step 2.2: fz-classify -- deterministic failure classification

- Map exit codes to oracle categories
- Detect panic patterns in stderr
- Detect hang (timeout)
- Detect segfault (signal-based exit codes)
- Deduplication by (exit_code, stderr_signature)
- Unit tests for each classification path

### Step 2.3: fz-artifacts -- report generation

- Write JSON reports
- Write Markdown human-readable reports
- Write corpus seed files
- Create artifacts directory structure
- Unit tests for report format correctness

### Step 2.4: fz-cli -- basic CLI skeleton

- clap subcommands: targets run, campaigns report
- Wire up manifest -> corpus -> exec -> classify -> artifacts pipeline
- End-to-end test: run against trivial test binary, verify report generated

## Phase 3: Mutation Engine

**Goal**: Systematic coverage expansion via deterministic mutations.

### Step 3.1: fz-mutate -- mutation operators

- Byte flip, bit flip
- Token/line deletion, duplication
- Splice (insert substring from another case)
- Nesting growth, numeric boundary substitution
- Delimiter confusion, encoding variation
- Property tests: mutated input differs from original, preserves validity where applicable

### Step 3.2: Mutation integration with campaigns

- Add mutation layer to campaign pipeline
- Configurable mutation budget per case
- Retain interesting mutations (new exit code, new stderr)
- Integration test: mutation discovers crash in test binary

## Phase 4: LLM Integration

**Goal**: AI-generated seeds and crash analysis via Ollama.

### Step 4.1: fz-llm -- Ollama client

- HTTP client for Ollama API (generate endpoint)
- JSON schema enforcement on responses
- Retry with exponential backoff
- Timeout handling
- Fallback behavior when Ollama is unavailable
- Unit tests with mock HTTP server

### Step 4.2: Prompt templates

- Edge-case input generation prompts (per target type)
- Crash analysis prompts
- Grammar inference prompts
- Template unit tests: correct variable substitution, no empty prompts

### Step 4.3: LLM seed integration

- Wire LLM seeds into campaign pipeline (Layer 2)
- Validate LLM-generated inputs before execution
- Track provenance (which cases came from LLM vs baseline vs mutation)
- Integration test: LLM seeds produce valid inputs (mocked LLM)

## Phase 5: Feedback Loop and Promotion

**Goal**: Close the loop -- retain interesting cases, minimize, promote to tests.

### Step 5.1: Feedback loop

- Track interestingness: new exit code, new stderr, longer runtime
- Feed interesting cases back into mutation pool
- Prevent infinite loops (max feedback iterations)
- Integration test: feedback discovers progressively more failures

### Step 5.2: Case minimization

- Binary search on input to find smallest reproducer
- Preserve failure signature during minimization
- Unit tests: minimized case is smaller, still crashes

### Step 5.3: Regression test promotion

- Template-based Rust test generation
- Generate #[test] function from case record
- Write to configurable output directory
- Human review: write to separate directory, never auto-commit

### Step 5.4: Campaign reporting

- Aggregate stats across all layers
- Crash count, unique failures, promoted artifacts
- Budget utilization per layer
- Timeline of discoveries
- Full fz-cli campaign subcommand

## Phase 6: Polish and Documentation

**Goal**: Production-ready tool.

- README with usage examples
- Example target manifests for common scenarios
- Example test binary for trying fuzzit
- CI-ready: all tests pass, zero warnings
- sw-checklist compliance
