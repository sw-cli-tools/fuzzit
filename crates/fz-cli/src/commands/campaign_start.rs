use std::collections::HashSet;
use std::path::Path;

use anyhow::Context;
use fz_classify::{classify, signature};
use fz_core::{CampaignReport, CaseRecord, Classification, Provenance};
use fz_corpus::{CaseInput, generate_baseline_corpus};
use fz_exec::execute;
use fz_llm::{OllamaClient, build_seed_prompt};
use fz_manifest::parse_manifest;

struct CampaignState {
    findings: Vec<CaseRecord>,
    known_signatures: HashSet<u64>,
    panic_count: usize,
    hang_count: usize,
    crash_count: usize,
    total_executions: usize,
}

impl CampaignState {
    fn new() -> Self {
        Self {
            findings: Vec::new(),
            known_signatures: HashSet::new(),
            panic_count: 0,
            hang_count: 0,
            crash_count: 0,
            total_executions: 0,
        }
    }

    fn record(
        &mut self,
        input: &[u8],
        result: fz_core::ExecutionResult,
        classification: Classification,
        provenance: Provenance,
    ) {
        self.total_executions += 1;
        match classification {
            Classification::Panic => self.panic_count += 1,
            Classification::Hang => self.hang_count += 1,
            Classification::Segfault => self.crash_count += 1,
            _ => {}
        }

        let sig = signature(&result);
        if matches!(
            classification,
            Classification::Panic | Classification::Hang | Classification::Segfault
        ) && !self.known_signatures.contains(&sig)
        {
            self.known_signatures.insert(sig);
            self.findings.push(CaseRecord {
                input: input.to_vec(),
                result,
                classification,
                provenance,
            });
        }
    }
}

fn run_layer(
    target: &fz_core::FuzzTarget,
    inputs: &[CaseInput],
    state: &mut CampaignState,
    provenance: Provenance,
) {
    for case_input in inputs {
        let result = match execute(target, &case_input.data) {
            Ok(r) => r,
            Err(_) => {
                state.total_executions += 1;
                continue;
            }
        };

        let classification = classify(&result, &target.oracle, target.timeout_ms);
        state.record(&case_input.data, result, classification, provenance);
    }
}

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
            let result = match execute(target, &mutated) {
                Ok(r) => r,
                Err(_) => {
                    state.total_executions += 1;
                    count += 1;
                    continue;
                }
            };

            let classification = classify(&result, &target.oracle, target.timeout_ms);
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

        let prev_findings = state.findings.len();
        run_layer(target, &inputs, state, Provenance::Feedback);

        if state.findings.len() == prev_findings {
            break;
        }
    }
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

    // Layer 1: Baseline
    eprintln!("fuzzit: layer 1 - baseline corpus...");
    let mut corpus = generate_baseline_corpus();
    corpus.truncate(baseline_budget);
    run_layer(&target, &corpus, &mut state, Provenance::Baseline);
    eprintln!(
        "  executions: {}, findings: {}",
        corpus.len(),
        state.findings.len()
    );

    // Layer 2: LLM seeds
    eprintln!("fuzzit: layer 2 - LLM seeds...");
    let client = OllamaClient::new(llm_model);
    if client.is_available() {
        let prompt = build_seed_prompt(&target, 20);
        match client.generate(&prompt) {
            Ok(response) => {
                let llm_inputs: Vec<CaseInput> = response
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .filter(|line| line.len() < 1_048_576)
                    .map(|line| CaseInput {
                        data: line.as_bytes().to_vec(),
                        description: format!("llm: {}", &line[..line.len().min(60)]),
                    })
                    .collect();

                eprintln!("  got {} inputs from LLM", llm_inputs.len());
                run_layer(&target, &llm_inputs, &mut state, Provenance::Llm);
                eprintln!(
                    "  executions: {}, findings: {}",
                    llm_inputs.len(),
                    state.findings.len()
                );
            }
            Err(e) => {
                eprintln!("  LLM generation failed, skipping: {e}");
            }
        }
    } else {
        eprintln!("  Ollama not available, skipping LLM layer");
    }

    // Layer 3: Mutation
    eprintln!("fuzzit: layer 3 - mutation...");
    let mutation_inputs: Vec<CaseInput> = corpus.to_vec();
    mutation_layer(&target, &mutation_inputs, mutation_budget, &mut state);
    eprintln!(
        "  mutations: {}, findings: {}",
        mutation_budget,
        state.findings.len()
    );

    // Layer 4: Feedback
    eprintln!("fuzzit: layer 4 - feedback...");
    feedback_layer(&target, &mut state, 3);
    eprintln!("  findings: {}", state.findings.len());

    // Report
    let report = CampaignReport {
        target_name: target.name.clone(),
        total_executions: state.total_executions,
        crash_count: state.crash_count,
        hang_count: state.hang_count,
        panic_count: state.panic_count,
        unique_failures: state.findings.len(),
        findings: state.findings,
    };

    let output_dir = std::path::PathBuf::from("artifacts").join(format!(
        "run_{}",
        chrono::Local::now().format("%Y-%m-%d_%H%M%S")
    ));

    fz_artifacts::init_output_dir(&output_dir)?;
    for (i, finding) in report.findings.iter().enumerate() {
        let _ = fz_artifacts::write_case(&output_dir, i, &finding.input, finding);
    }
    fz_artifacts::write_report(&output_dir, &report)?;

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
