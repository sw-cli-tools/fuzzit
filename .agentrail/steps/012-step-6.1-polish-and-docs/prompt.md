Phase 6, Step 6.1: Polish, documentation, and production readiness

Final cleanup pass to make fuzzit production-ready.

Tasks:
1. Write README.md with:
   - Project description and purpose
   - Installation instructions (cargo install or build from source)
   - Quick start example with a sample manifest
   - Full CLI reference (all subcommands and flags)
   - Example target manifests for common scenarios (compiler, API, REPL)
   - Architecture overview (link to docs/architecture.md)

2. Create example manifests in examples/ directory:
   - examples/compiler.toml (for a C/Rust compiler)
   - examples/api.toml (for a JSON API)
   - examples/repl.toml (for a REPL)

3. Create a test target binary for trying fuzzit:
   - examples/test_target.rs -- a simple program that reads stdin and has known bugs:
     - panics on input starting with 'panic'
     - hangs on input 'loop'
     - crashes on very long input (stack overflow)

4. Verify full quality gate:
   - cargo fmt --all
   - cargo clippy --all-targets --all-features -- -D warnings
   - cargo test (all pass)
   - sw-checklist (all pass)

5. Update docs/architecture.md if anything changed during implementation.
6. Update docs/design.md if any new decisions were made.
7. Update docs/plan.md to mark completed steps.

This is the final step of the saga.