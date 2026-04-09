use fz_artifacts::{
    init_output_dir, promote_batch, promote_to_test, write_case, write_corpus_seed, write_report,
};
use fz_core::{CampaignReport, CaseRecord, Classification, ExecutionResult, Provenance};

fn sample_report() -> CampaignReport {
    CampaignReport {
        target_name: "test-target".into(),
        target_kind: "Cli".into(),
        target_entry: "/bin/test".into(),
        timeout_ms: 2000,
        total_budget: 50,
        total_executions: 10,
        crash_count: 2,
        hang_count: 1,
        panic_count: 1,
        unique_failures: 2,
        promoted_count: 1,
        promoted_dir: "tests/fuzzit/".into(),
        findings: vec![CaseRecord {
            input: b"panic input".to_vec(),
            result: ExecutionResult {
                exit_code: Some(101),
                stdout: vec![],
                stderr: "thread panicked".into(),
                wall_time_ms: 5,
                killed: false,
            },
            classification: Classification::Panic,
            provenance: Provenance::Baseline,
            discovered_at: "2026-04-08T12:00:00".into(),
        }],
        baseline_stats: fz_core::LayerStats {
            executions: 10,
            new_findings: 1,
        },
        ..Default::default()
    }
}

fn sample_case_record() -> CaseRecord {
    CaseRecord {
        input: b"crash data".to_vec(),
        result: ExecutionResult {
            exit_code: Some(101),
            stdout: vec![],
            stderr: "panicked".into(),
            wall_time_ms: 3,
            killed: false,
        },
        classification: Classification::Panic,
        provenance: Provenance::Baseline,
        discovered_at: String::new(),
    }
}

fn sample_target() -> fz_core::FuzzTarget {
    fz_core::FuzzTarget {
        name: "test-parser".into(),
        kind: fz_core::TargetKind::Cli,
        entry: std::path::PathBuf::from("/bin/test"),
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

fn sample_case(input: &[u8], classification: Classification) -> CaseRecord {
    CaseRecord {
        input: input.to_vec(),
        result: ExecutionResult {
            exit_code: Some(1),
            stdout: vec![],
            stderr: String::new(),
            wall_time_ms: 5,
            killed: false,
        },
        classification,
        provenance: Provenance::Baseline,
        discovered_at: "2026-04-08T12:00:00".into(),
    }
}

#[test]
fn init_creates_dirs() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("run_001");
    let result = init_output_dir(&dir);
    assert!(result.is_ok());
    assert!(dir.join("cases").exists());
    assert!(dir.join("crashes").exists());
    assert!(dir.join("corpus").exists());
}

#[test]
fn init_creates_nested_parent() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("a").join("b").join("c");
    let result = init_output_dir(&dir);
    assert!(result.is_ok());
    assert!(dir.exists());
}

#[test]
fn write_report_creates_files() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let report = sample_report();
    let result = write_report(&dir, &report);
    assert!(result.is_ok());
    assert!(dir.join("report.json").exists());
    assert!(dir.join("report.md").exists());
}

#[test]
fn json_report_is_valid() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let report = sample_report();
    write_report(&dir, &report).unwrap();
    let content = std::fs::read_to_string(dir.join("report.json")).unwrap();
    let parsed: CampaignReport = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.target_name, "test-target");
    assert_eq!(parsed.total_executions, 10);
}

#[test]
fn markdown_report_contains_sections() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let report = sample_report();
    write_report(&dir, &report).unwrap();
    let content = std::fs::read_to_string(dir.join("report.md")).unwrap();
    assert!(content.contains("test-target"));
    assert!(content.contains("Target"));
    assert!(content.contains("Configuration"));
    assert!(content.contains("Summary"));
    assert!(content.contains("Total executions: 10"));
    assert!(content.contains("Layer Breakdown"));
    assert!(content.contains("Baseline"));
    assert!(content.contains("Mutation"));
    assert!(content.contains("Findings"));
    assert!(content.contains("[Panic]"));
    assert!(content.contains("via Baseline"));
    assert!(content.contains("Recommendations"));
}

#[test]
fn write_case_creates_file() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let record = sample_case_record();
    write_case(&dir, 1, b"crash data", &record).unwrap();
    assert!(dir.join("cases/case_0001.txt").exists());
    let content = std::fs::read(dir.join("cases/case_0001.txt")).unwrap();
    assert_eq!(content, b"crash data");
}

#[test]
fn write_case_creates_metadata() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let record = sample_case_record();
    write_case(&dir, 1, b"crash data", &record).unwrap();
    assert!(dir.join("cases/case_0001.meta.json").exists());
    let content = std::fs::read_to_string(dir.join("cases/case_0001.meta.json")).unwrap();
    let parsed: CaseRecord = serde_json::from_str(&content).unwrap();
    assert_eq!(parsed.classification, Classification::Panic);
}

#[test]
fn write_case_copies_crash_to_crashes_dir() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let record = sample_case_record();
    write_case(&dir, 1, b"crash data", &record).unwrap();
    assert!(dir.join("crashes/case_0001.txt").exists());
}

#[test]
fn write_case_no_crash_copy_for_success() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let record = CaseRecord {
        input: b"ok".to_vec(),
        result: ExecutionResult {
            exit_code: Some(0),
            stdout: vec![],
            stderr: String::new(),
            wall_time_ms: 1,
            killed: false,
        },
        classification: Classification::Success,
        provenance: Provenance::Baseline,
        discovered_at: String::new(),
    };
    write_case(&dir, 2, b"ok", &record).unwrap();
    assert!(!dir.join("crashes/case_0002.txt").exists());
}

#[test]
fn write_corpus_seed_creates_file() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    write_corpus_seed(&dir, 0, b"seed data").unwrap();
    assert!(dir.join("corpus/seed_0000.bin").exists());
    let content = std::fs::read(dir.join("corpus/seed_0000.bin")).unwrap();
    assert_eq!(content, b"seed data");
}

#[test]
fn write_corpus_seed_multiple_indices() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    write_corpus_seed(&dir, 0, b"a").unwrap();
    write_corpus_seed(&dir, 99, b"z").unwrap();
    assert!(dir.join("corpus/seed_0000.bin").exists());
    assert!(dir.join("corpus/seed_0099.bin").exists());
}

#[test]
fn markdown_report_empty_findings() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = init_output_dir(tmp.path()).unwrap();
    let report = CampaignReport {
        target_name: "clean".into(),
        target_kind: "Cli".into(),
        target_entry: "/bin/clean".into(),
        timeout_ms: 1000,
        total_budget: 5,
        total_executions: 5,
        crash_count: 0,
        hang_count: 0,
        panic_count: 0,
        unique_failures: 0,
        promoted_count: 0,
        promoted_dir: String::new(),
        findings: vec![],
        ..Default::default()
    };
    write_report(&dir, &report).unwrap();
    let content = std::fs::read_to_string(dir.join("report.md")).unwrap();
    assert!(content.contains("clean"));
    assert!(!content.contains("## Findings"));
    assert!(content.contains("Layer Breakdown"));
    assert!(!content.contains("Recommendations"));
}

#[test]
fn promote_creates_file() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"crash input", Classification::Panic);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    assert!(result.exists());
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains("fn fuzzit_test_parser_0000()"));
    assert!(content.contains("Auto-generated fuzzit regression test"));
}

#[test]
fn test_name_derived_from_target() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"input", Classification::Panic);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains("test_parser"));
}

#[test]
fn input_literal_escaped() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"hello \"world\"", Classification::Panic);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains(r#"hello \"world\""#));
}

#[test]
fn backslash_escaped() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"path\\to\\file", Classification::Panic);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains(r#"path\\to\\file"#));
}

#[test]
fn invalid_utf8_uses_byte_literal() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(&[0xC0, 0x80], Classification::Panic);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains(r#"b"\xC0\x80""#));
}

#[test]
fn metadata_in_comments() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"input", Classification::Segfault);
    let result = promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(&result).unwrap();
    assert!(content.contains("// Classification: Segfault"));
    assert!(content.contains("// Provenance: Baseline"));
    assert!(content.contains("// Discovered: 2026-04-08T12:00:00"));
}

#[test]
fn promote_batch_creates_multiple() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let cases = vec![
        sample_case(b"a", Classification::Panic),
        sample_case(b"b", Classification::Hang),
        sample_case(b"c", Classification::Segfault),
    ];
    let paths = promote_batch(&dir, &target, &cases).unwrap();
    assert_eq!(paths.len(), 3);
    let content = std::fs::read_to_string(&paths[0]).unwrap();
    assert!(content.contains("fn fuzzit_test_parser_0000()"));
    assert!(content.contains("fn fuzzit_test_parser_0002()"));
}

#[test]
fn output_dir_created() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("nested/deep/output");
    let target = sample_target();
    let case = sample_case(b"input", Classification::Panic);
    promote_to_test(&dir, &target, &case, 0).unwrap();
    assert!(dir.exists());
}

#[test]
fn promote_multiple_appends() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case1 = sample_case(b"first", Classification::Panic);
    let case2 = sample_case(b"second", Classification::Hang);
    promote_to_test(&dir, &target, &case1, 0).unwrap();
    promote_to_test(&dir, &target, &case2, 1).unwrap();
    let content = std::fs::read_to_string(dir.join("fuzzit_regressions.rs")).unwrap();
    assert!(content.contains("fn fuzzit_test_parser_0000()"));
    assert!(content.contains("fn fuzzit_test_parser_0001()"));
}

#[test]
fn promote_duplicate_skipped() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("tests");
    let target = sample_target();
    let case = sample_case(b"same", Classification::Panic);
    promote_to_test(&dir, &target, &case, 0).unwrap();
    promote_to_test(&dir, &target, &case, 0).unwrap();
    let content = std::fs::read_to_string(dir.join("fuzzit_regressions.rs")).unwrap();
    let count = content.matches("fn fuzzit_test_parser_0000(").count();
    assert_eq!(count, 1);
}
