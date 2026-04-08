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

    lines.push("## Target".to_string());
    lines.push(String::new());
    lines.push(format!("- Name: {}", report.target_name));
    lines.push(format!("- Kind: {}", report.target_kind));
    lines.push(format!("- Entry: {}", report.target_entry));
    lines.push(format!("- Timeout: {}ms", report.timeout_ms));
    lines.push(String::new());

    lines.push("## Configuration".to_string());
    lines.push(String::new());
    lines.push(format!("- Total budget: {}", report.total_budget));
    lines.push(format!("- Total executions: {}", report.total_executions));
    lines.push(String::new());

    lines.push("## Summary".to_string());
    lines.push(String::new());
    lines.push(format!("- Crashes: {}", report.crash_count));
    lines.push(format!("- Panics: {}", report.panic_count));
    lines.push(format!("- Hangs: {}", report.hang_count));
    lines.push(format!("- Unique failures: {}", report.unique_failures));
    lines.push(format!("- Promoted to tests: {}", report.promoted_count));
    if !report.promoted_dir.is_empty() {
        lines.push(format!("- Test output: {}", report.promoted_dir));
    }
    lines.push(String::new());

    lines.push("## Layer Breakdown".to_string());
    lines.push(String::new());
    lines.push("| Layer | Executions | New Findings |".to_string());
    lines.push("|-------|------------|-------------|".to_string());
    lines.push(format!(
        "| Baseline | {} | {} |",
        report.baseline_stats.executions, report.baseline_stats.new_findings
    ));
    lines.push(format!(
        "| LLM | {} | {} |",
        report.llm_stats.executions, report.llm_stats.new_findings
    ));
    lines.push(format!(
        "| Mutation | {} | {} |",
        report.mutation_stats.executions, report.mutation_stats.new_findings
    ));
    lines.push(format!(
        "| Feedback | {} | {} |",
        report.feedback_stats.executions, report.feedback_stats.new_findings
    ));
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
            let timestamp = if finding.discovered_at.is_empty() {
                String::from("N/A")
            } else {
                finding.discovered_at.clone()
            };
            lines.push(format!(
                "{}. [{:?}] {} (via {:?}, {})",
                i + 1,
                finding.classification,
                truncated,
                finding.provenance,
                timestamp
            ));
        }
    }

    if report.unique_failures > 0 {
        lines.push(String::new());
        lines.push("## Recommendations".to_string());
        lines.push(String::new());
        lines.push(format!(
            "- {} crash(es) found, {} promoted to regression tests",
            report.unique_failures, report.promoted_count
        ));
        if report.mutation_stats.new_findings == 0 && report.baseline_stats.executions > 0 {
            lines.push("- Consider increasing mutation budget for deeper exploration".to_string());
        }
        if report.llm_stats.executions == 0 {
            lines.push("- Enable Ollama for LLM-generated seed diversity".to_string());
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
}
