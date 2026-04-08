# AGENTS.md

This file provides guidance to AI coding agents (opencode, Claude Code, Gemini CLI, etc.) when working with code in this repository. It is the agentrail equivalent of a CLAUDE.md file.

## Project: fuzzit -- LLM-Guided Fuzz Testing Tool

Rust CLI tool that combines deterministic fuzzing, AI-generated edge cases,
mutation engines, and feedback loops to discover crashes, hangs, panics, and
unexpected behavior in text-input programs (compilers, interpreters, REPLs,
CLI tools, APIs).

## CRITICAL: AgentRail Session Protocol (MUST follow exactly)

Every agent session follows this 6-step loop. Do NOT skip or reorder steps.

### 1. START (do this FIRST, before anything else)
```bash
agentrail next
```
Read the output carefully. It contains your current step, prompt, plan context, and any relevant skills/trajectories.

### 2. BEGIN (immediately after reading the next output)
```bash
agentrail begin
```

### 3. WORK (do what the step prompt says)
Do NOT ask "want me to proceed?" or "shall I start?". The step prompt IS your instruction. Execute it directly.

### 4. PRE-COMMIT QUALITY GATE (MANDATORY -- every step, no exceptions)
Every completed saga step must be high quality, documented, and pushed to GitHub.
If anything fails, fix the underlying problem -- NEVER suppress, allow, or work around a check.

#### Phase A: Rust quality + documentation
1. `cargo fmt --all` -- Format all Rust code
2. `cargo clippy --all-targets --all-features -- -D warnings` -- Zero warnings.
   If clippy reports a warning, fix the code. Do NOT add `#[allow(...)]` or
   change the clippy invocation. The underlying problem must be resolved.
3. `cargo test` -- All tests pass. Fix any failures before proceeding.
4. Review staged files with `git status` -- No build artifacts, no unintended files
5. Verify documentation is up-to-date -- if code changed affected public APIs,
   components, or behavior, update relevant doc comments and any docs/ files
6. **Commit** the formatted, clippy-clean, test-passing, documented code now.

#### Phase B: sw-checklist conformance
7. `sw-checklist` -- Fix all FAIL and WARN items to avoid tech-debt accumulation.
   You may commit the Phase A code before fixing sw-checklist issues.
   If you have questions about how to fix a specific FAIL/WARN, ask for help.
   Re-run `sw-checklist` after fixes to confirm clean.
8. **Commit** the sw-checklist fixes.

#### Phase B+: Re-verify if code changed
9. If Phase B made code changes, re-run Phase A steps 1-3 (fmt, clippy, test).
   Fix any regressions. Commit if needed.

#### Phase C: Push
10. `git push` -- Every completed step must be pushed to GitHub

### 5. COMPLETE (LAST thing, after committing and pushing)
```bash
agentrail complete --summary "what you accomplished" \
  --reward 1 \
  --actions "tools and approach used"
```
- If the step failed: `--reward -1 --failure-mode "what went wrong"`
- If the saga is finished: add `--done`

### 6. STOP (after complete, DO NOT continue working)
Do NOT make further code changes after running `agentrail complete`.
Any changes after complete are untracked and invisible to the next session.
Future work belongs in the NEXT step, not this one.

## Key Rules

- **Do NOT skip steps** -- the next session depends on accurate tracking
- **Do NOT ask for permission** -- the step prompt is the instruction
- **Do NOT continue working** after `agentrail complete`
- **Commit before complete** -- always commit first, then record completion
- **NO Python** -- this is a Rust project only. No venvs, no pip, no python3.
- **TDD required** -- write failing test first, then implement, then refactor
- **No warning suppression** -- never use `#[allow(...)]` to hide clippy warnings
- **Draft-only artifacts** -- fuzz findings are never auto-committed or auto-merged

## Architecture

Workspace with 9 crates:

```
crates/
  fz-core/        shared types (FuzzTarget, ExecutionResult, CaseRecord)
  fz-manifest/    TOML target manifest parsing
  fz-corpus/      baseline edge corpus generation
  fz-exec/        sandboxed target execution
  fz-classify/    deterministic failure classification
  fz-mutate/      mutation operators
  fz-llm/         Ollama LLM client for seed generation
  fz-artifacts/   JSON + Markdown report writer
  fz-cli/         binary entrypoint (clap subcommands)
```

See docs/architecture.md for full details.

## Key Design Decisions

- Child process execution only (never in-process) for safety
- Ollama-first LLM integration (local/LAN, no cloud dependency)
- Template-based harness generation (model fills holes, not arbitrary code)
- Deterministic classification before any LLM use
- Budget-based campaigns for predictable runtime
- Deduplication by failure signature, not by input content

See docs/design.md for full decision record.

## Build

```bash
cargo build                    # Build all crates
cargo test                     # Run all tests
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt --all                # Format
```

## CLI

```
fuzzit targets run --manifest <path>
fuzzit targets generate --manifest <path> --budget <n>
fuzzit campaigns start --manifest <path> --budget <n>
fuzzit campaigns report --dir <path>
fuzzit cases minimize --case <path>
fuzzit cases promote --case <path>
```

## Useful Commands

```bash
agentrail status          # Current saga state
agentrail history         # All completed steps
agentrail plan            # View the plan
agentrail next            # Current step + context
```

## Scaling Rules

When a module or crate grows too large, split it. Never consolidate.

| Signal | Action |
|--------|--------|
| Too many functions in a module | Add a new module |
| Too many modules in a crate | Add a new crate |
| Too many crates | Add a new top-level component directory |

### Code Style

- File size limit: 500 lines (prefer 200-300)
- Function size limit: 50 lines (prefer 10-30)
- Max 7 functions per module, 7 modules per crate
- Pure functions preferred, composed structs for data
- Every module with logic gets a `#[cfg(test)] mod tests`
- Edition 2024, inline format arguments: `format!("{name}")`

## Related Projects

- `sw-cli-tools` workspace (parent): other CLI tools
- `sw-embed` ecosystem: target programs to fuzz (compilers, interpreters, etc.)
- `agentrail`: saga management tool used for development workflow
