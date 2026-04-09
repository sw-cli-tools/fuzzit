use std::process::Command;

fn sample_target() -> fz_core::FuzzTarget {
    fz_core::FuzzTarget {
        name: "demo-parser".into(),
        kind: fz_core::TargetKind::Cli,
        entry: std::path::PathBuf::from("/bin/cat"),
        input_mode: fz_core::InputMode::Stdin,
        timeout_ms: 2000,
        oracle: fz_core::Oracle {
            success_exit_codes: vec![0],
            failure_exit_codes: vec![1],
            capture_stderr: true,
        },
        expectations: fz_core::Expectations {
            must_not_panic: true,
            must_not_hang: true,
            must_not_segfault: true,
        },
        seed_files: vec![],
        strategy: fz_core::Strategy { styles: vec![] },
    }
}

fn sample_case(input: &[u8], classification: fz_core::Classification) -> fz_core::CaseRecord {
    fz_core::CaseRecord {
        input: input.to_vec(),
        result: fz_core::ExecutionResult {
            exit_code: Some(101),
            stdout: vec![],
            stderr: "panicked".into(),
            wall_time_ms: 5,
            killed: false,
        },
        classification,
        provenance: fz_core::Provenance::Baseline,
        discovered_at: "2026-04-08T12:00:00".into(),
    }
}

#[test]
fn promoted_file_compiles_as_rust() {
    let dir = tempfile::tempdir().unwrap();
    let target = sample_target();
    let cases = vec![
        sample_case(b"panic trigger", fz_core::Classification::Panic),
        sample_case(b"loop", fz_core::Classification::Hang),
        sample_case(b"crash", fz_core::Classification::Segfault),
        sample_case(&[0xC0, 0x80], fz_core::Classification::Segfault),
        sample_case(br#"hello "world""#, fz_core::Classification::Panic),
        sample_case(b"path\\to\\file", fz_core::Classification::Panic),
    ];

    let paths = fz_artifacts::promote_batch(dir.path(), &target, &cases).unwrap();
    assert_eq!(paths.len(), 6);

    let content = std::fs::read_to_string(&paths[0]).unwrap();
    assert!(content.contains("fn fuzzit_demo_parser_0000()"));
    assert!(content.contains("fn fuzzit_demo_parser_0005()"));

    let out_dir = tempfile::tempdir().unwrap();
    let output = Command::new("rustc")
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "lib",
            "--emit=metadata",
            "--out-dir",
        ])
        .arg(out_dir.path())
        .arg(&paths[0])
        .output()
        .expect("failed to run rustc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "promoted file failed to compile:\n{stderr}\nFile content:\n{content}"
    );
}

#[test]
fn promoted_file_contains_all_test_names() {
    let dir = tempfile::tempdir().unwrap();
    let target = sample_target();
    let cases = vec![
        sample_case(b"a", fz_core::Classification::Panic),
        sample_case(b"b", fz_core::Classification::Hang),
    ];

    let path = fz_artifacts::promote_to_test(dir.path(), &target, &cases[0], 0).unwrap();
    let path2 = fz_artifacts::promote_to_test(dir.path(), &target, &cases[1], 1).unwrap();
    assert_eq!(path, path2);

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("fn fuzzit_demo_parser_0000()"));
    assert!(content.contains("fn fuzzit_demo_parser_0001()"));
}

#[test]
fn promoted_file_has_metadata_comments() {
    let dir = tempfile::tempdir().unwrap();
    let target = sample_target();
    let case = sample_case(b"test", fz_core::Classification::Segfault);

    let path = fz_artifacts::promote_to_test(dir.path(), &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&path).unwrap();

    assert!(content.contains("// Target: demo-parser"));
    assert!(content.contains("// Classification: Segfault"));
    assert!(content.contains("// Provenance: Baseline"));
    assert!(content.contains("// Discovered: 2026-04-08T12:00:00"));
    assert!(content.contains("// Input preview: test"));
}
