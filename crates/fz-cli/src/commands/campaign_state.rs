use std::collections::HashSet;

use fz_classify::{classify, signature};
use fz_core::{CaseRecord, Classification, Provenance};
use fz_corpus::CaseInput;
use fz_exec::execute;

pub struct CampaignState {
    pub findings: Vec<CaseRecord>,
    known_signatures: HashSet<u64>,
    pub panic_count: usize,
    pub hang_count: usize,
    pub crash_count: usize,
    pub total_executions: usize,
}

impl CampaignState {
    pub fn new() -> Self {
        Self {
            findings: Vec::new(),
            known_signatures: HashSet::new(),
            panic_count: 0,
            hang_count: 0,
            crash_count: 0,
            total_executions: 0,
        }
    }

    pub fn record(
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
                discovered_at: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            });
        }
    }
}

pub fn run_layer(
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
