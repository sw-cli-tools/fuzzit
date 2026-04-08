use std::collections::HashSet;
use std::path::Path;

use anyhow::Context;
use fz_classify::{classify, signature};
use fz_core::{CampaignReport, CaseRecord, Classification, Provenance};
use fz_corpus::generate_baseline_corpus;
use fz_exec::execute;
use fz_manifest::parse_manifest;

pub fn run_targets(manifest_path: &Path) -> anyhow::Result<()> {
    let target = parse_manifest(manifest_path)
        .with_context(|| format!("failed to load manifest: {}", manifest_path.display()))?;

    eprintln!("fuzzit: target '{}' ({:?})", target.name, target.kind);
    eprintln!("fuzzit: generating baseline corpus...");

    let corpus = generate_baseline_corpus();
    let oracle = &target.oracle;
    let timeout_ms = target.timeout_ms;

    eprintln!("fuzzit: executing {} inputs...", corpus.len());

    let mut findings: Vec<CaseRecord> = Vec::new();
    let mut known_signatures: HashSet<u64> = HashSet::new();
    let mut panic_count = 0usize;
    let mut hang_count = 0usize;
    let mut crash_count = 0usize;

    for (i, case_input) in corpus.iter().enumerate() {
        let result = match execute(&target, &case_input.data) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  [{}] execution error: {e}", i + 1);
                continue;
            }
        };

        let classification = classify(&result, oracle, timeout_ms);
        let sig = signature(&result);

        if matches!(
            classification,
            Classification::Panic | Classification::Hang | Classification::Segfault
        ) && !known_signatures.contains(&sig)
        {
            known_signatures.insert(sig);
            findings.push(CaseRecord {
                input: case_input.data.clone(),
                result,
                classification,
                provenance: Provenance::Baseline,
                discovered_at: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            });
        }

        match classification {
            Classification::Panic => panic_count += 1,
            Classification::Hang => hang_count += 1,
            Classification::Segfault => crash_count += 1,
            _ => {}
        }

        if matches!(
            classification,
            Classification::Panic | Classification::Hang | Classification::Segfault
        ) && !known_signatures.contains(&sig)
        {
            eprintln!(
                "  [{}] NEW {:?}: {}",
                i + 1,
                classification,
                case_input.description
            );
        }
    }

    let unique_failures = findings.len();
    let total_executions = corpus.len();

    let report = CampaignReport {
        target_name: target.name.clone(),
        target_kind: format!("{:?}", target.kind),
        target_entry: target.entry.display().to_string(),
        timeout_ms: target.timeout_ms,
        total_budget: corpus.len(),
        total_executions,
        crash_count,
        hang_count,
        panic_count,
        unique_failures: findings.len(),
        promoted_count: 0,
        promoted_dir: String::new(),
        findings: findings.clone(),
        baseline_stats: fz_core::LayerStats {
            executions: corpus.len(),
            new_findings: findings.len(),
        },
        ..Default::default()
    };

    let output_dir = std::path::PathBuf::from("artifacts").join(format!(
        "run_{}",
        chrono::Local::now().format("%Y-%m-%d_%H%M%S")
    ));

    fz_artifacts::init_output_dir(&output_dir)?;
    fz_artifacts::write_report(&output_dir, &report)?;

    eprintln!();
    eprintln!("Campaign complete:");
    eprintln!("  Executions: {}", total_executions);
    eprintln!("  Panics:     {}", panic_count);
    eprintln!("  Hangs:      {}", hang_count);
    eprintln!("  Crashes:    {}", crash_count);
    eprintln!("  Unique:     {}", unique_failures);
    eprintln!("  Report:     {}/report.md", output_dir.display());

    Ok(())
}
