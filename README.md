# fuzzit

LLM-guided fuzz testing tool for CLI programs, compilers, interpreters, REPLs, and APIs.

fuzzit combines deterministic baseline fuzzing, AI-generated edge cases via Ollama,
mutation engines, and feedback loops to discover crashes, hangs, panics, and
unexpected behavior in text-input programs.

## Features

- **Deterministic baseline corpus** -- empty inputs, whitespace, delimiters, escapes, invalid UTF-8, numeric boundaries, deep nesting
- **LLM seed generation** -- targeted edge cases from local Ollama models (no cloud dependency)
- **Mutation engine** -- byte flip, bit flip, token delete/duplicate, splice, nesting growth, delimiter confusion
- **Feedback loop** -- retains interesting cases and feeds them back into the mutation pool
- **Safe execution** -- all targets run in child processes with wall-time timeouts
- **Deterministic classification** -- panic, hang, segfault, unexpected exit, unexpected stderr
- **Regression test promotion** -- converts findings into Rust `#[test]` functions for human review
- **Campaign reporting** -- JSON and Markdown reports with per-layer statistics

## Installation

```bash
# Build from source
cargo build --release

# The binary will be at target/release/fuzzit
```

Requires Rust 2024 edition toolchain.

Optional: [Ollama](https://ollama.ai) running locally for LLM seed generation (Layer 2).
Without Ollama, campaigns skip LLM seeds and run the remaining layers.

## Quick Start

### 1. Create a target manifest

```toml
# my_parser.toml
[target]
name = "my-parser"
kind = "cli"
entry = "./target/debug/my-parser"
input_mode = "stdin"
timeout_ms = 2000

[oracle]
success_exit_codes = [0]
failure_exit_codes = [101, 134, 137, 139]

[expectations]
must_not_panic = true
must_not_hang = true
must_not_segfault = true
```

### 2. Run a baseline test

```bash
fuzzit targets run --manifest my_parser.toml
```

### 3. Run a full campaign

```bash
fuzzit campaigns start --manifest my_parser.toml --budget 500
```

### 4. View results

```bash
fuzzit campaigns report --dir artifacts/run_2026-04-08_001
```

## CLI Reference

### `fuzzit targets run`

Execute a fuzz target with the deterministic baseline corpus.

```
fuzzit targets run --manifest <path>
```

Outputs to `artifacts/run_<timestamp>/` with JSON and Markdown reports.

### `fuzzit targets generate`

Generate seeds for a fuzz target (not yet implemented).

```
fuzzit targets generate --manifest <path> --budget <n>
```

### `fuzzit campaigns start`

Start a multi-layer fuzz campaign. Layers:

| Layer | Description | Budget share |
|-------|-------------|-------------|
| 1. Baseline | Deterministic edge cases | 30% |
| 2. LLM Seeds | Ollama-generated targeted inputs | 10% |
| 3. Mutation | Deterministic mutations | 40% |
| 4. Feedback | Re-run interesting findings | 20% |

```
fuzzit campaigns start --manifest <path> --budget <n>
```

- `--budget` defaults to 500
- LLM model defaults to `llama3`

### `fuzzit campaigns report`

Display a campaign report from an artifacts directory.

```
fuzzit campaigns report --dir <path>
```

## Manifest Reference

A target manifest is a TOML file with these sections:

### `[target]` (required)

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Target name (must be non-empty) |
| `kind` | string | `cli`, `api`, or `repl` |
| `entry` | string | Path to the target binary |
| `input_mode` | string | `stdin`, `args`, or `file` |
| `timeout_ms` | integer | Wall-time timeout in milliseconds (must be > 0) |

### `[oracle]` (required, fields optional)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `success_exit_codes` | [int] | `[]` | Exit codes that indicate success |
| `failure_exit_codes` | [int] | `[]` | Exit codes that indicate known failure |
| `capture_stderr` | bool | `true` | Whether to capture stderr output |

### `[expectations]` (required, fields optional)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `must_not_panic` | bool | `true` | Flag panic as a finding |
| `must_not_hang` | bool | `true` | Flag timeout as a finding |
| `must_not_segfault` | bool | `true` | Flag segfault as a finding |

### `[seeds]` (optional)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `files` | [string] | `[]` | Paths to seed input files |

### `[strategy]` (optional)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `styles` | [string] | `[]` | Mutation style hints |

## Example Manifests

See the `examples/` directory for ready-to-use manifests:

- `examples/compiler.toml` -- fuzzing a C/Rust compiler
- `examples/api.toml` -- fuzzing a JSON API endpoint
- `examples/repl.toml` -- fuzzing an interactive REPL

## Example Test Target

A test target with known bugs is included at `examples/test_target.sh`:

- Panics on input starting with `panic`
- Hangs on input `loop`
- Crashes on very long input (stack overflow)

## Architecture

fuzzit is a workspace of 9 crates:

| Crate | Role |
|-------|------|
| `fz-core` | Shared types (FuzzTarget, ExecutionResult, CaseRecord) |
| `fz-manifest` | TOML manifest parsing and validation |
| `fz-corpus` | Baseline edge corpus generation |
| `fz-exec` | Sandboxed child process execution |
| `fz-classify` | Deterministic failure classification |
| `fz-mutate` | Mutation operators and case minimization |
| `fz-llm` | Ollama LLM client and prompt templates |
| `fz-artifacts` | JSON/Markdown reports and regression test promotion |
| `fz-cli` | Binary entrypoint with clap subcommands |

See [docs/architecture.md](docs/architecture.md) for the full architecture document.

## Build

```bash
cargo build                    # Build all crates
cargo test                     # Run all tests (155 tests)
cargo clippy --all-targets --all-features -- -D warnings  # Zero warnings
cargo fmt --all                # Format
```

## License

Private -- part of the sw-cli-tools workspace.
