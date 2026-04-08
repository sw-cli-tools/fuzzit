use std::path::Path;

use anyhow::Context;
use fz_core::CampaignReport;

pub fn init_output_dir(base: &Path) -> anyhow::Result<std::path::PathBuf> {
    std::fs::create_dir_all(base)
        .with_context(|| format!("failed to create output dir: {}", base.display()))?;
    let cases_dir = base.join("cases");
    std::fs::create_dir_all(&cases_dir)
        .with_context(|| format!("failed to create cases dir: {}", cases_dir.display()))?;
    let crashes_dir = base.join("crashes");
    std::fs::create_dir_all(&crashes_dir)
        .with_context(|| format!("failed to create crashes dir: {}", crashes_dir.display()))?;
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
    use fz_core::{CaseRecord, Classification, ExecutionResult, Provenance};

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

    #[test]
    fn init_creates_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("run_001");
        let result = init_output_dir(&dir);
        assert!(result.is_ok());
        assert!(dir.join("cases").exists());
        assert!(dir.join("crashes").exists());
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
    }
}
