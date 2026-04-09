use std::io::Write as IoWrite;
use std::path::PathBuf;

use fz_core::{Expectations, FuzzTarget, InputMode, Oracle, Strategy, TargetKind};
use fz_exec::execute;

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
