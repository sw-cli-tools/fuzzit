Phase 1, Step 1.1: Workspace scaffold

Convert the single-package Cargo.toml to a workspace root with 9 crates:
fz-core, fz-manifest, fz-corpus, fz-exec, fz-classify, fz-mutate, fz-llm, fz-artifacts, fz-cli.

Steps:
1. Rewrite root Cargo.toml as a workspace with [workspace.members] listing all crates/crates/*/
2. Create crates/ directory
3. Create each crate subdirectory with Cargo.toml (appropriate name, edition 2024, lib=true except fz-cli which is bin=true)
4. Create src/lib.rs (empty module) in each lib crate, src/main.rs in fz-cli
5. Add workspace-level dependencies: anyhow, serde, toml, clap, reqwest, tempfile
6. Verify cargo build, cargo test, cargo clippy all pass
7. Create a trivial test binary at crates/fz-cli/tests/test_target.rs that reads from stdin and prints it back (for end-to-end testing later)
8. Update .gitignore if needed

TDD: write a simple test in fz-core first, then scaffold.

See docs/architecture.md for crate layout, docs/plan.md for full plan.