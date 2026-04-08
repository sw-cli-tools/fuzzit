use std::io::Write;
use std::process::Stdio;
use std::time::Instant;

use anyhow::Context;
use fz_core::{ExecutionResult, FuzzTarget, InputMode};

pub fn execute(target: &FuzzTarget, input: &[u8]) -> anyhow::Result<ExecutionResult> {
    let mut cmd = std::process::Command::new(&target.entry);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let temp_file;
    match target.input_mode {
        InputMode::Stdin => {}
        InputMode::Args => {
            let input_str = std::str::from_utf8(input)
                .map_err(|_| anyhow::anyhow!("args input_mode requires valid UTF-8"))?;
            cmd.arg(input_str);
        }
        InputMode::File => {
            temp_file =
                tempfile::NamedTempFile::new().context("failed to create temp file for input")?;
            temp_file
                .as_file()
                .write_all(input)
                .context("failed to write input to temp file")?;
            cmd.arg(temp_file.path());
        }
    }

    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn target: {}", target.entry.display()))?;

    if matches!(target.input_mode, InputMode::Stdin)
        && let Some(ref mut stdin) = child.stdin
    {
        stdin
            .write_all(input)
            .context("failed to write to target stdin")?;
    }
    drop(child.stdin.take());

    let start = Instant::now();
    let output = child
        .wait_with_output()
        .context("failed to collect target output")?;
    let wall_time_ms = start.elapsed().as_millis() as u64;

    Ok(ExecutionResult {
        exit_code: output.status.code(),
        stdout: output.stdout,
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        wall_time_ms,
        killed: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::{Expectations, Oracle, Strategy, TargetKind};
    use std::io::Write as IoWrite;
    use std::path::PathBuf;

    fn make_target(entry: PathBuf, input_mode: InputMode, timeout_ms: u64) -> FuzzTarget {
        FuzzTarget {
            name: "test".into(),
            kind: TargetKind::Cli,
            entry,
            input_mode,
            timeout_ms,
            oracle: Oracle {
                success_exit_codes: vec![0],
                failure_exit_codes: vec![101, 134],
                capture_stderr: true,
            },
            expectations: Expectations {
                must_not_panic: true,
                must_not_hang: true,
                must_not_segfault: true,
            },
            seed_files: vec![],
            strategy: Strategy { styles: vec![] },
        }
    }

    fn shell_script(tmp: &tempfile::TempDir, name: &str, script: &str) -> PathBuf {
        let bin = tmp.path().join(name);
        let mut f = std::fs::File::create(&bin).unwrap();
        writeln!(f, "#!/bin/sh").unwrap();
        writeln!(f, "{script}").unwrap();
        std::process::Command::new("chmod")
            .args(["+x", bin.to_str().unwrap()])
            .status()
            .unwrap();
        bin
    }

    #[test]
    fn stdin_echo() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "echo_cat", "cat");
        let target = make_target(bin, InputMode::Stdin, 5000);
        let result = execute(&target, b"hello world").unwrap();
        assert_eq!(result.stdout, b"hello world");
        assert_eq!(result.exit_code, Some(0));
        assert!(!result.killed);
    }

    #[test]
    fn args_input() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "echo_args", "echo \"$1\"");
        let target = make_target(bin, InputMode::Args, 5000);
        let result = execute(&target, b"my argument").unwrap();
        let output = String::from_utf8_lossy(&result.stdout);
        assert!(output.contains("my argument"));
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn file_input() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "echo_file", "cat \"$1\"");
        let target = make_target(bin, InputMode::File, 5000);
        let result = execute(&target, b"file content").unwrap();
        assert_eq!(result.stdout, b"file content");
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn exit_code_captured() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "exit_42", "exit 42");
        let target = make_target(bin, InputMode::Stdin, 5000);
        let result = execute(&target, b"").unwrap();
        assert_eq!(result.exit_code, Some(42));
    }

    #[test]
    fn empty_input() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "echo_empty", "cat");
        let target = make_target(bin, InputMode::Stdin, 5000);
        let result = execute(&target, b"").unwrap();
        assert_eq!(result.exit_code, Some(0));
        assert!(!result.killed);
    }

    #[test]
    fn large_input() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "echo_large", "cat");
        let target = make_target(bin, InputMode::Stdin, 5000);
        let input = vec![b'X'; 65_536];
        let result = execute(&target, &input).unwrap();
        assert_eq!(result.stdout, input);
        assert_eq!(result.exit_code, Some(0));
    }

    #[test]
    fn stderr_captured() {
        let tmp = tempfile::tempdir().unwrap();
        let bin = shell_script(&tmp, "stderr_wr", "echo 'error msg' >&2");
        let target = make_target(bin, InputMode::Stdin, 5000);
        let result = execute(&target, b"").unwrap();
        assert!(result.stderr.contains("error msg"));
    }

    #[test]
    fn non_existent_binary_errors() {
        let target = make_target(
            PathBuf::from("/nonexistent/binary/path"),
            InputMode::Stdin,
            5000,
        );
        let result = execute(&target, b"test");
        assert!(result.is_err());
    }
}
