use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use fz_core::{CaseRecord, FuzzTarget};

pub fn promote_to_test(
    output_dir: &Path,
    target: &FuzzTarget,
    case: &CaseRecord,
    index: usize,
) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create output dir: {}", output_dir.display()))?;

    let test_name = sanitize_test_name(&target.name, index);
    let input_literal = format_input_literal(&case.input);
    let classification_str = format!("{:?}", case.classification);
    let provenance_str = format!("{:?}", case.provenance);
    let timestamp = if case.discovered_at.is_empty() {
        "unknown".to_string()
    } else {
        case.discovered_at.clone()
    };
    let preview = format_input_preview(&case.input, 80);

    let content = format!(
        "//! Auto-generated fuzzit regression test\n\
        //! Target: {name}\n\
        //! Classification: {classification}\n\
        //! Provenance: {provenance}\n\
        //! Discovered: {timestamp}\n\
        //! Input preview: {preview}\n\n\
        #[test]\n\
        fn {test_name}() {{\n\
    let input = {input_literal};\n\
    // This test documents a fuzzit finding.\n\
    // Wire up actual target invocation to make it a real regression test.\n\
    // Expected: should not {expected_behavior}\n\
    // TODO: replace this assert with actual target invocation.\n\
    assert!(false, \"fuzzit regression placeholder: run `fuzzit` to reproduce\");\n\
}}\n",
        name = target.name,
        classification = classification_str,
        provenance = provenance_str,
        timestamp = timestamp,
        preview = preview,
        test_name = test_name,
        input_literal = input_literal,
        expected_behavior = expected_behavior(&case.classification),
    );

    let path = output_dir.join("fuzzit_regressions.rs");
    if path.exists() {
        let existing = std::fs::read_to_string(&path)?;
        if !existing.contains(&format!("fn {test_name}()")) {
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .open(&path)
                .context("failed to open regression file")?;
            file.write_all(content.as_bytes())?;
        }
    } else {
        std::fs::write(&path, content.as_bytes()).context("failed to write regression file")?;
    }

    Ok(path)
}

pub fn promote_batch(
    output_dir: &Path,
    target: &FuzzTarget,
    cases: &[CaseRecord],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        let path = promote_to_test(output_dir, target, case, i)?;
        paths.push(path);
    }
    Ok(paths)
}

fn sanitize_test_name(target_name: &str, index: usize) -> String {
    let base: String = target_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("fuzzit_{base}_{index:04}")
}

fn format_input_literal(input: &[u8]) -> String {
    match std::str::from_utf8(input) {
        Ok(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("\"{escaped}\"")
        }
        Err(_) => {
            let bytes: Vec<String> = input.iter().map(|b| format!("\\x{b:02X}")).collect();
            format!("b\"{}\"", bytes.join(""))
        }
    }
}

fn format_input_preview(input: &[u8], max_len: usize) -> String {
    let s = String::from_utf8_lossy(input);
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

fn expected_behavior(classification: &fz_core::Classification) -> &'static str {
    use fz_core::Classification;
    match classification {
        Classification::Success => "fail",
        Classification::Panic => "panic",
        Classification::Hang => "hang",
        Classification::Segfault => "segfault",
        Classification::UnexpectedExit => "fail with unexpected exit code",
        Classification::UnexpectedStderr => "produce unexpected stderr",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::{Classification, ExecutionResult, Provenance};

    fn sample_target() -> FuzzTarget {
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
        assert!(content.contains("//! Classification: Segfault"));
        assert!(content.contains("//! Provenance: Baseline"));
        assert!(content.contains("//! Discovered: 2026-04-08T12:00:00"));
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
}
