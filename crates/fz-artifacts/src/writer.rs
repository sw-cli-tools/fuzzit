use std::path::Path;

use anyhow::Context;
use fz_core::{CampaignReport, CaseRecord};

use super::report_format::format_report;

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
