Phase 1, Step 1.2: fz-core shared types

Implement the core types crate at crates/fz-core/.

Types to define:
- FuzzTarget: name (String), kind (TargetKind: Cli/Api/Repl), entry (PathBuf), input_mode (InputMode: Stdin/Args/File), timeout_ms (u64), oracle (Oracle), expectations (Expectations), seed_files (Vec<PathBuf>), strategy (Strategy)
- ExecutionResult: exit_code (Option<i32>), stdout (Vec<u8>), stderr (String), wall_time_ms (u64), killed (bool)
- CaseRecord: input (Vec<u8>), result (ExecutionResult), classification (Classification), provenance (Provenance: Baseline/Llm/Mutation/Feedback)
- CampaignConfig: manifest_path (PathBuf), budget (usize), output_dir (PathBuf), layer_budgets (Option<LayerBudgets>)
- CampaignReport: target_name, total_executions, crash_count, unique_failures, promoted_count, per_layer_stats
- Oracle: success_exit_codes (Vec<i32>), failure_exit_codes (Vec<i32>), capture_stderr (bool)
- Expectations: must_not_panic, must_not_hang, must_not_segfault (all bool)
- Classification: Success, Panic, Hang, Segfault, UnexpectedExit, UnexpectedStderr
- Provenance: Baseline, Llm, Mutation, Feedback, UserSeed
- TargetKind, InputMode enums

Use serde derive for serialization. Use anyhow for errors.

TDD: write failing tests for type construction, validation (e.g., empty name, zero timeout), and serde round-trip first. Then implement.

Keep src/lib.rs under 50 lines (re-export from submodules if needed).
Max 7 functions per module. Split into types.rs, error.rs modules.