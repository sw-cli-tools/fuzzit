# Architecture

## Overview

fuzzit is an LLM-guided fuzz testing tool for CLI programs, compilers, interpreters,
REPLs, APIs, and any tool that accepts text input. It combines deterministic
baseline fuzzing, AI-generated edge cases, mutation engines, and feedback loops
to discover crashes, hangs, panics, and unexpected behavior.

## Design Philosophy

- Deterministic Rust code owns execution, scoring, sandboxing, and reporting
- LLM (Ollama, local/LAN) provides targeted creativity: grammar inference,
  edge-case synthesis, crash minimization
- Every discovered failure becomes a permanent regression asset
- Manifest-driven: targets are described declaratively, not imperatively

## High-Level Pipeline

```
Target Manifest --> Corpus Builder --> Executor --> Classifier --> Artifact Writer
                          ^                |
                          |                v
                    Mutation Engine <-- Feedback Loop
                          ^
                          |
                       LLM Seeds
```

## Crate Layout

```
fuzzit/
  Cargo.toml              (workspace root)
  crates/
    fz-core/              shared types (FuzzTarget, ExecutionResult, CaseRecord, CampaignConfig)
    fz-manifest/          TOML target manifest parsing and validation
    fz-corpus/            baseline edge corpus generation (empty, whitespace, huge, escapes, etc.)
    fz-exec/              sandboxed target execution (process spawn, timeout, resource limits)
    fz-classify/          deterministic failure classification (exit code, stderr, stack traces)
    fz-mutate/            deterministic mutation operators (byte flip, token delete, splice, etc.)
    fz-llm/               Ollama/OpenAI-compatible LLM client for seed generation and crash analysis
    fz-artifacts/         JSON + Markdown report and regression test writer
    fz-cli/               binary entrypoint with clap subcommands
```

## Component Responsibilities

### fz-core

Central types shared across all crates:

- `FuzzTarget` -- parsed from manifest: name, entry point, input mode, timeout, oracle rules
- `ExecutionResult` -- exit code, stdout, stderr, wall time, killed status
- `CaseRecord` -- input data, result, classification, provenance (baseline/llm/mutation/feedback)
- `CampaignConfig` -- budget, strategy selection, output directory
- `CampaignReport` -- aggregate stats, crash count, unique failures, promoted artifacts

### fz-manifest

Parses and validates TOML target manifests:

```toml
[target]
name = "my-lexer"
kind = "cli"              # cli, api, repl
entry = "./target/debug/my-lexer"
input_mode = "stdin"      # stdin, args, file
timeout_ms = 2000

[oracle]
success_exit_codes = [0]
failure_exit_codes = [101, 134, 137, 139]
capture_stderr = true

[expectations]
must_not_panic = true
must_not_hang = true
must_not_segfault = true

[seeds]
files = ["seeds/valid_01.txt", "seeds/invalid_01.txt"]

[strategy]
styles = ["grammarish", "mutation", "boundary", "encoding", "escape-heavy"]
```

### fz-corpus

Generates a deterministic baseline corpus of edge-case inputs:

- empty string, whitespace only, single characters
- huge input (configurable size)
- repeated delimiters, escape sequences
- invalid UTF-8, UTF-16-like noise, null bytes
- CRLF / CR / LF variations
- negative / max / min integer strings
- deep nesting, weird identifiers

### fz-exec

Sandboxed target execution:

- spawns target as child process with input piped via stdin/args/file
- enforces wall-time timeout (kills on exceed)
- captures stdout, stderr, exit code
- resource limits where OS supports it (rlimit)
- no network access, no secrets in environment

### fz-classify

Deterministic first-pass failure classification:

- maps exit code to oracle categories
- detects panic patterns in stderr (thread panicked, stack backtrace)
- detects hang (timeout exceeded)
- detects segfault (signal-based exit codes: 134=SIGABRT, 139=SIGSEGV, 137=SIGKILL)
- deduplication by (exit_code, stderr_signature) tuple

### fz-mutate

Deterministic mutation operators for expanding corpus:

- byte flip, bit flip
- token/line deletion, duplication
- splice (insert substring from another case)
- nesting growth (wrap in delimiters/brackets)
- numeric boundary substitution (i64::MAX, 0, -1, etc.)
- delimiter confusion (swap quotes, brackets)
- encoding variation (inject invalid UTF-8 sequences)

### fz-llm

LLM integration for targeted seed generation:

- Ollama backend (primary, local/LAN)
- Optional OpenAI-compatible backend for future use
- Prompt templates for:
  - edge-case input generation (per target type: compiler, API, REPL, etc.)
  - crash analysis and minimization suggestions
  - grammar inference from seed corpus
  - harness code generation (Rust unit tests, proptest strategies)
- JSON schema enforcement on responses
- Retry with exponential backoff
- Response validation before use

### fz-artifacts

Output generation:

- JSON reports for machine consumption
- Markdown reports for human review
- Regression test files (Rust #[test] functions)
- Corpus seed files (for cargo-fuzz / libFuzzer)
- Replay manifests (input + expected oracle outcome)
- Minimized reproducers

### fz-cli

Binary entrypoint using clap:

```
fuzzit targets run --manifest targets/my-parser.toml
fuzzit targets generate --manifest targets/my-parser.toml --budget 100
fuzzit campaigns start --manifest targets/my-parser.toml --budget 500
fuzzit cases minimize --case artifacts/cases/case_0042.txt
fuzzit cases promote --case artifacts/cases/case_0042.txt
fuzzit campaigns report --dir artifacts/run_2026-04-08_001
```

## Fuzzing Layers

### Layer 1: Deterministic Baseline

Always runs first. No LLM needed. Catches many issues immediately.

### Layer 2: LLM-Generated Seeds

Asks coding model to produce targeted edge cases based on target type
and existing corpus. Low frequency, high creativity.

### Layer 3: Mutation Engine

Deterministic Rust mutations applied to all retained cases.
High frequency, systematic exploration.

### Layer 4: Feedback Loop

Retains "interesting" cases (new exit code, new stderr, longer runtime,
new stack trace) and feeds them back into mutation pool.

## Safety Model

- All target execution in child processes (never in-process)
- No auto-merge, no auto-commit, no auto-PR
- All outputs are "draft" artifacts requiring human review to promote
- No secrets in environment, no network access for targets
- LLM-generated test code validated structurally before execution
- Template-based harness generation (model fills holes, not arbitrary code)

## Data Flow

```
manifest.toml
     |
     v
  fz-manifest --> FuzzTarget
                       |
     +-----------------+------------------+
     |                 |                  |
     v                 v                  v
  fz-corpus      fz-llm seeds     user seed files
  (baseline)     (targeted)       (provided)
     |                 |                  |
     +--------+--------+--------+---------+
              |                  |
              v                  v
         fz-exec            fz-mutate
         (execute)          (mutate)
              |                  |
              v                  |
         fz-classify            |
         (classify)             |
              |                  |
              +--------+---------+
                       |
                       v
                  fz-artifacts
                  (report + promote)
```
