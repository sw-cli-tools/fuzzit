use std::path::Path;

use anyhow::Context;
use fz_core::{CampaignReport, CaseRecord};

pub fn init_output_dir(base: &Path) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(base)
        .with_context(|| format!("failed to create output dir: {}", base.display()))?;
    let cases_dir = base.join("cases");
    std::fs::create_dir_all(&cases_dir)
        .with_context(|| format!("failed to create cases dir: {}", cases_dir.display()))?;
    let crashes_dir = base.join("crashes");
    std::fs::create_dir_all(&crashes_dir)
        .with_context(|| format!("failed to create crashes dir: {}", crashes_dir.display()))?;
    let corpus_dir = base.join("corpus");
    std::fs::create_dir_all(&corpus_dir)
        .with_context(|| format!("failed to create corpus dir: {}", corpus_dir.display()))?;
    Ok(base.to_path_buf())
}

pub fn write_report(dir: &Path, report: &CampaignReport) -> anyhow::Result<()> {
    let json_path = dir.join("report.json");
    let json =
        serde_json::to_string_pretty(report).context("failed to serialize report to JSON")?;
    std::fs::write(&json_path, json)
        .with_context(|| format!("failed to write report to {}", json_path.display()))?;

    let md_path = dir.join("report.md");
    let md = format_report(report);
    std::fs::write(&md_path, md)
        .with_context(|| format!("failed to write report to {}", md_path.display()))?;

    Ok(())
}

pub fn write_case(
    dir: &Path,
    index: usize,
    input: &[u8],
    record: &CaseRecord,
) -> anyhow::Result<()> {
    let filename = format!("case_{index:04}.txt");
    let path = dir.join("cases").join(&filename);
    std::fs::write(&path, input)
        .with_context(|| format!("failed to write case file: {}", path.display()))?;

    let meta_path = dir.join("cases").join(format!("case_{index:04}.meta.json"));
    let meta = serde_json::to_string_pretty(record).context("failed to serialize case metadata")?;
    std::fs::write(&meta_path, meta)
        .with_context(|| format!("failed to write case metadata: {}", meta_path.display()))?;

    if matches!(
        record.classification,
        fz_core::Classification::Panic
            | fz_core::Classification::Segfault
            | fz_core::Classification::Hang
    ) {
        let crash_path = dir.join("crashes").join(&filename);
        std::fs::write(&crash_path, input)
            .with_context(|| format!("failed to write crash file: {}", crash_path.display()))?;
    }

    Ok(())
}

pub fn write_corpus_seed(dir: &Path, index: usize, input: &[u8]) -> anyhow::Result<()> {
    let filename = format!("seed_{index:04}.bin");
    let path = dir.join("corpus").join(&filename);
    std::fs::write(&path, input)
        .with_context(|| format!("failed to write corpus seed: {}", path.display()))?;
    Ok(())
}

fn format_report(report: &CampaignReport) -> String {
    let mut lines = Vec::new();
    lines.push(format!("# Fuzz Campaign Report: {}", report.target_name));
    lines.push(String::new());
    lines.push("## Summary".to_string());
    lines.push(String::new());
    lines.push(format!("- Total executions: {}", report.total_executions));
    lines.push(format!("- Crashes: {}", report.crash_count));
    lines.push(format!("- Panics: {}", report.panic_count));
    lines.push(format!("- Hangs: {}", report.hang_count));
    lines.push(format!("- Unique failures: {}", report.unique_failures));
    lines.push(String::new());

    if !report.findings.is_empty() {
        lines.push("## Findings".to_string());
        lines.push(String::new());
        for (i, finding) in report.findings.iter().enumerate() {
            let preview = String::from_utf8_lossy(&finding.input);
            let truncated = if preview.len() > 80 {
                format!("{}...", &preview[..80])
            } else {
                preview.to_string()
            };
            lines.push(format!(
                "{}. [{:?}] {}",
                i + 1,
                finding.classification,
                truncated
            ));
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::{Classification, ExecutionResult, Provenance};

    fn sample_report() -> CampaignReport {
        CampaignReport {
            target_name: "test-target".into(),
            total_executions: 10,
            crash_count: 2,
            hang_count: 1,
            panic_count: 1,
            unique_failures: 2,
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
            }],
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
        assert!(content.contains("Summary"));
        assert!(content.contains("Total executions: 10"));
        assert!(content.contains("Findings"));
        assert!(content.contains("[Panic]"));
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
            total_executions: 5,
            crash_count: 0,
            hang_count: 0,
            panic_count: 0,
            unique_failures: 0,
            findings: vec![],
        };
        write_report(&dir, &report).unwrap();
        let content = std::fs::read_to_string(dir.join("report.md")).unwrap();
        assert!(content.contains("clean"));
        assert!(!content.contains("Findings"));
    }
}
