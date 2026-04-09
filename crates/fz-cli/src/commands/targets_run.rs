use std::collections::HashSet;
use std::path::Path;

use anyhow::Context;
use fz_classify::{classify, signature};
use fz_core::{CampaignReport, CaseRecord, Classification, Provenance};
use fz_corpus::generate_baseline_corpus;
use fz_exec::execute;
use fz_manifest::parse_manifest;

struct RunState {
    findings: Vec<CaseRecord>,
    known_signatures: HashSet<u64>,
    panic_count: usize,
    hang_count: usize,
    crash_count: usize,
}

fn record_finding(
    state: &mut RunState,
    case_input: &fz_corpus::CaseInput,
    result: fz_core::ExecutionResult,
    oracle: &fz_core::Oracle,
    timeout_ms: u64,
) {
    let classification = classify(&result, oracle, timeout_ms);
    match classification {
        Classification::Panic => state.panic_count += 1,
        Classification::Hang => state.hang_count += 1,
        Classification::Segfault => state.crash_count += 1,
        _ => {}
    }
    let sig = signature(&result);
    if matches!(
        classification,
        Classification::Panic | Classification::Hang | Classification::Segfault
    ) && !state.known_signatures.contains(&sig)
    {
        state.known_signatures.insert(sig);
        state.findings.push(CaseRecord {
            input: case_input.data.clone(),
            result,
            classification,
            provenance: Provenance::Baseline,
            discovered_at: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
        });
    }
}

fn build_run_report(
    target: &fz_core::FuzzTarget,
    corpus_len: usize,
    state: &RunState,
) -> CampaignReport {
    CampaignReport {
        target_name: target.name.clone(),
        target_kind: format!("{:?}", target.kind),
        target_entry: target.entry.display().to_string(),
        timeout_ms: target.timeout_ms,
        total_budget: corpus_len,
        total_executions: corpus_len,
        crash_count: state.crash_count,
        hang_count: state.hang_count,
        panic_count: state.panic_count,
        unique_failures: state.findings.len(),
        promoted_count: 0,
        promoted_dir: String::new(),
        findings: state.findings.clone(),
        baseline_stats: fz_core::LayerStats {
            executions: corpus_len,
            new_findings: state.findings.len(),
        },
        ..Default::default()
    }
}

fn write_run_output(
    corpus_len: usize,
    state: &RunState,
    report: &CampaignReport,
) -> anyhow::Result<()> {
    let output_dir = std::path::PathBuf::from("artifacts").join(format!(
        "run_{}",
        chrono::Local::now().format("%Y-%m-%d_%H%M%S")
    ));
    fz_artifacts::init_output_dir(&output_dir)?;
    fz_artifacts::write_report(&output_dir, report)?;
    eprintln!();
    eprintln!("Campaign complete:");
    eprintln!("  Executions: {}", corpus_len);
    eprintln!("  Panics:     {}", state.panic_count);
    eprintln!("  Hangs:      {}", state.hang_count);
    eprintln!("  Crashes:    {}", state.crash_count);
    eprintln!("  Unique:     {}", state.findings.len());
    eprintln!("  Report:     {}/report.md", output_dir.display());
    Ok(())
}

pub fn run_targets(manifest_path: &Path) -> anyhow::Result<()> {
    let target = parse_manifest(manifest_path)
        .with_context(|| format!("failed to load manifest: {}", manifest_path.display()))?;

    eprintln!("fuzzit: target '{}' ({:?})", target.name, target.kind);
    eprintln!("fuzzit: generating baseline corpus...");

    let corpus = generate_baseline_corpus();
    eprintln!("fuzzit: executing {} inputs...", corpus.len());

    let mut state = RunState {
        findings: Vec::new(),
        known_signatures: HashSet::new(),
        panic_count: 0,
        hang_count: 0,
        crash_count: 0,
    };

    for (i, case_input) in corpus.iter().enumerate() {
        let result = match execute(&target, &case_input.data) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  [{}] execution error: {e}", i + 1);
                continue;
            }
        };
        record_finding(
            &mut state,
            case_input,
            result,
            &target.oracle,
            target.timeout_ms,
        );
    }

    let report = build_run_report(&target, corpus.len(), &state);
    write_run_output(corpus.len(), &state, &report)
}
