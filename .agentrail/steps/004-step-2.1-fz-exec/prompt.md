Phase 2, Step 2.1: fz-exec sandboxed target execution

Implement the sandboxed target execution crate at crates/fz-exec/.

Responsibilities:
- Spawn target binary as child process
- Deliver input via stdin, args, or temp file (based on InputMode)
- Enforce wall-time timeout (kill process if exceeded)
- Capture stdout, stderr, exit code
- Set resource limits where OS supports (rlimit)
- No network access, no secrets in environment

Public API:
- fn execute(target: &FuzzTarget, input: &[u8]) -> anyhow::Result<ExecutionResult>

Implementation notes:
- Use std::process::Command
- For stdin mode: write input to child's stdin, then close
- For args mode: pass input as a single argument (UTF-8 string)
- For file mode: write input to temp file, pass path as argument
- Use process::Child::try_wait or thread + sleep for timeout
- On timeout, kill the child and mark as killed=true
- Set stdout/stderr to piped for capture

TDD tests (use a trivial test binary):
- Execute with stdin input, verify output captured
- Execute with args input, verify argument received
- Execute with file input, verify file created
- Timeout kills process and returns killed=true
- Exit code captured correctly
- Empty input handled
- Large input handled
- Non-existent binary returns clear error

Dependencies: fz-core, anyhow, tempfile.