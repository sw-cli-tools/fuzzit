Phase 2, Step 2.2: fz-classify deterministic failure classification

Implement the deterministic classification crate at crates/fz-classify/.

Responsibilities:
- Classify ExecutionResult into a Classification enum
- Detect panics (stderr contains 'thread panicked' or 'stack backtrace')
- Detect hangs (result.killed == true && wall_time >= timeout)
- Detect segfaults (exit code 134=SIGABRT, 139=SIGSEGV, 137=SIGKILL)
- Detect unexpected exit codes (not in oracle success/failure lists)
- Deduplicate cases by (exit_code, stderr_signature_hash)

Public API:
- fn classify(result: &ExecutionResult, oracle: &Oracle, timeout_ms: u64) -> Classification
- fn signature(result: &ExecutionResult) -> u64 (hash of exit_code + stderr)
- fn is_interesting(new_result: &ExecutionResult, known_signatures: &HashSet<u64>) -> bool

TDD tests:
- Exit code 0 with clean stderr -> Success
- Stderr contains 'thread panicked' -> Panic
- Killed + timeout exceeded -> Hang
- Exit code 134 -> Segfault
- Exit code 139 -> Segfault
- Exit code 1 (not in oracle) -> UnexpectedExit
- Signature is deterministic: same input produces same hash
- Signature differs for different stderr content
- is_interesting returns true for new signature, false for known

Dependencies: fz-core, anyhow.