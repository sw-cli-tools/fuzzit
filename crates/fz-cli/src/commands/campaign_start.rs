use std::path::Path;

use anyhow::Context;
use fz_core::{CampaignReport, LayerStats, Provenance};
use fz_corpus::{CaseInput, generate_baseline_corpus};
use fz_llm::{OllamaClient, build_seed_prompt};
use fz_manifest::parse_manifest;

use super::campaign_state::{CampaignState, run_layer};

fn mutation_layer(
    target: &fz_core::FuzzTarget,
    inputs: &[CaseInput],
    budget: usize,
    state: &mut CampaignState,
) {
    let mut rng = rand::rng();
    let mut count = 0;
    for case_input in inputs {
        while count < budget {
            let mutated = fz_mutate::mutate(&case_input.data, &mut rng);
            let result = match fz_exec::execute(target, &mutated) {
                Ok(r) => r,
                Err(_) => {
                    state.total_executions += 1;
                    count += 1;
                    continue;
                }
            };
            let classification = fz_classify::classify(&result, &target.oracle, target.timeout_ms);
            state.record(&mutated, result, classification, Provenance::Mutation);
            count += 1;
        }
        if count >= budget {
            break;
        }
    }
}

fn feedback_layer(target: &fz_core::FuzzTarget, state: &mut CampaignState, max_iterations: usize) {
    for _ in 0..max_iterations {
        let inputs: Vec<CaseInput> = state
            .findings
            .iter()
            .filter(|f| f.provenance != Provenance::Feedback)
            .map(|f| CaseInput {
                data: f.input.clone(),
                description: format!("feedback_{:?}", f.classification),
            })
            .collect();
        if inputs.is_empty() {
            break;
        }
        let prev = state.findings.len();
        run_layer(target, &inputs, state, Provenance::Feedback);
        if state.findings.len() == prev {
            break;
        }
    }
}

fn llm_seed_layer(target: &fz_core::FuzzTarget, state: &mut CampaignState, llm_model: &str) {
    eprintln!("fuzzit: layer 2 - LLM seeds...");
    let client = OllamaClient::new(llm_model);
    if !client.is_available() {
        eprintln!("  Ollama not available, skipping LLM layer");
        return;
    }
    let prompt = build_seed_prompt(target, 20);
    let response = match client.generate(&prompt) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("  LLM generation failed, skipping: {e}");
            return;
        }
    };
    let llm_inputs: Vec<CaseInput> = response
        .lines()
        .filter(|l| !l.trim().is_empty() && l.len() < 1_048_576)
        .map(|l| CaseInput {
            data: l.as_bytes().to_vec(),
            description: format!("llm: {}", &l[..l.len().min(60)]),
        })
        .collect();
    eprintln!("  got {} inputs from LLM", llm_inputs.len());
    run_layer(target, &llm_inputs, state, Provenance::Llm);
    eprintln!(
        "  executions: {}, findings: {}",
        llm_inputs.len(),
        state.findings.len()
    );
}

fn build_campaign_report(
    target: &fz_core::FuzzTarget,
    total_budget: usize,
    state: &CampaignState,
    baseline_budget: usize,
    mutation_budget: usize,
) -> CampaignReport {
    CampaignReport {
        target_name: target.name.clone(),
        target_kind: format!("{:?}", target.kind),
        target_entry: target.entry.display().to_string(),
        timeout_ms: target.timeout_ms,
        total_budget,
        total_executions: state.total_executions,
        crash_count: state.crash_count,
        hang_count: state.hang_count,
        panic_count: state.panic_count,
        unique_failures: state.findings.len(),
        promoted_count: 0,
        promoted_dir: String::new(),
        findings: state.findings.clone(),
        baseline_stats: LayerStats {
            executions: baseline_budget,
            new_findings: 0,
        },
        llm_stats: LayerStats::default(),
        mutation_stats: LayerStats {
            executions: mutation_budget,
            new_findings: 0,
        },
        feedback_stats: LayerStats::default(),
    }
}

fn finalize_campaign(report: &CampaignReport) -> anyhow::Result<()> {
    let output_dir = std::path::PathBuf::from("artifacts").join(format!(
        "run_{}",
        chrono::Local::now().format("%Y-%m-%d_%H%M%S")
    ));
    fz_artifacts::init_output_dir(&output_dir)?;
    for (i, f) in report.findings.iter().enumerate() {
        let _ = fz_artifacts::write_case(&output_dir, i, &f.input, f);
    }
    fz_artifacts::write_report(&output_dir, report)?;
    eprintln!();
    eprintln!("Campaign complete:");
    eprintln!("  Executions: {}", report.total_executions);
    eprintln!("  Panics:     {}", report.panic_count);
    eprintln!("  Hangs:      {}", report.hang_count);
    eprintln!("  Crashes:    {}", report.crash_count);
    eprintln!("  Unique:     {}", report.unique_failures);
    eprintln!("  Report:     {}/report.md", output_dir.display());
    Ok(())
}

pub fn start_campaign(
    manifest_path: &Path,
    total_budget: usize,
    llm_model: &str,
) -> anyhow::Result<()> {
    let target = parse_manifest(manifest_path)
        .with_context(|| format!("failed to load manifest: {}", manifest_path.display()))?;

    eprintln!(
        "fuzzit: campaign for '{}' ({:?}), budget={}",
        target.name, target.kind, total_budget
    );

    let mut state = CampaignState::new();
    let baseline_budget = total_budget * 30 / 100;
    let mutation_budget = total_budget * 40 / 100;

    eprintln!("fuzzit: layer 1 - baseline corpus...");
    let mut corpus = generate_baseline_corpus();
    corpus.truncate(baseline_budget);
    run_layer(&target, &corpus, &mut state, Provenance::Baseline);
    eprintln!(
        "  executions: {}, findings: {}",
        corpus.len(),
        state.findings.len()
    );

    llm_seed_layer(&target, &mut state, llm_model);

    eprintln!("fuzzit: layer 3 - mutation...");
    mutation_layer(&target, &corpus, mutation_budget, &mut state);
    eprintln!(
        "  mutations: {}, findings: {}",
        mutation_budget,
        state.findings.len()
    );

    eprintln!("fuzzit: layer 4 - feedback...");
    feedback_layer(&target, &mut state, 3);
    eprintln!("  findings: {}", state.findings.len());

    let report = build_campaign_report(
        &target,
        total_budget,
        &state,
        baseline_budget,
        mutation_budget,
    );
    finalize_campaign(&report)
}
