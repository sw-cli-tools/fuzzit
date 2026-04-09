use std::io::Write;
use std::path::PathBuf;

use fz_core::{Expectations, FuzzTarget, InputMode, Oracle, Strategy, TargetKind};
use fz_mutate::minimize;

fn make_crasher(dir: &tempfile::TempDir, exit_code: i32) -> PathBuf {
    let path = dir.path().join("crasher");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(f, "#!/bin/sh").unwrap();
    writeln!(f, "exit {exit_code}").unwrap();
    std::process::Command::new("chmod")
        .args(["+x", path.to_str().unwrap()])
        .status()
        .unwrap();
    path
}

fn make_target(entry: PathBuf) -> FuzzTarget {
    FuzzTarget {
        name: "test".into(),
        kind: TargetKind::Cli,
        entry,
        input_mode: InputMode::Stdin,
        timeout_ms: 5000,
        oracle: Oracle {
            success_exit_codes: vec![0],
            failure_exit_codes: vec![1],
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

#[test]
fn minimized_is_smaller_or_equal() {
    let tmp = tempfile::tempdir().unwrap();
    let target = make_target(make_crasher(&tmp, 1));
    let input = b"AAAA BBBB CCCC DDDD EEEE FFFF GGGG HHHH IIII JJJJ";
    let result = minimize(&target, input, 50).unwrap();
    assert!(result.len() <= input.len());
}

#[test]
fn non_crashing_returns_original() {
    let tmp = tempfile::tempdir().unwrap();
    let target = make_target(make_crasher(&tmp, 0));
    let input = b"hello world";
    let result = minimize(&target, input, 10).unwrap();
    assert_eq!(result, input);
}

#[test]
fn single_byte_crasher_returns_itself() {
    let tmp = tempfile::tempdir().unwrap();
    let target = make_target(make_crasher(&tmp, 1));
    let input = b"X";
    let result = minimize(&target, input, 10).unwrap();
    assert_eq!(result.len(), 1);
}

#[test]
fn max_iterations_respected() {
    let tmp = tempfile::tempdir().unwrap();
    let target = make_target(make_crasher(&tmp, 1));
    let input = vec![b'A'; 1000];
    let result = minimize(&target, &input, 1).unwrap();
    assert!(result.len() <= input.len());
}

#[test]
fn empty_input_returns_empty() {
    let tmp = tempfile::tempdir().unwrap();
    let target = make_target(make_crasher(&tmp, 1));
    let result = minimize(&target, b"", 10).unwrap();
    assert!(result.is_empty());
}
