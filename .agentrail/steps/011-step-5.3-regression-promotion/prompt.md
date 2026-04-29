Phase 5, Step 5.3: Regression test promotion

Implement promotion of fuzz findings to Rust regression tests.

Responsibilities:
- Generate Rust #[test] functions from crash cases
- Use template-based generation (model fills holes, not arbitrary code)
- Write to configurable output directory (never auto-commit)

Template:


Public API (in fz-artifacts):
- fn promote_to_test(output_dir: &Path, target: &FuzzTarget, case: &CaseRecord) -> anyhow::Result<PathBuf>
- fn promote_batch(output_dir: &Path, target: &FuzzTarget, cases: &[CaseRecord]) -> anyhow::Result<Vec<PathBuf>>

TDD tests:
- Generated test file compiles as valid Rust
- Test name is derived from target name and case index
- Input literal is correctly escaped (handles quotes, backslashes, null bytes)
- Multiple promotions create separate test functions in same file
- Output directory is created if it does not exist
- Test metadata (classification, provenance, timestamp) is in comments