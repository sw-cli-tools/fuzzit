use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TargetKind {
    Cli,
    Api,
    Repl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputMode {
    Stdin,
    Args,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Provenance {
    Baseline,
    Llm,
    Mutation,
    Feedback,
    UserSeed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Classification {
    Success,
    Panic,
    Hang,
    Segfault,
    UnexpectedExit,
    UnexpectedStderr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub wall_time_ms: u64,
    pub killed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseRecord {
    pub input: Vec<u8>,
    pub result: ExecutionResult,
    pub classification: Classification,
    pub provenance: Provenance,
    #[serde(default)]
    pub discovered_at: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerStats {
    pub executions: usize,
    pub new_findings: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CampaignReport {
    pub target_name: String,
    pub target_kind: String,
    pub target_entry: String,
    pub timeout_ms: u64,
    pub total_budget: usize,
    pub total_executions: usize,
    pub crash_count: usize,
    pub hang_count: usize,
    pub panic_count: usize,
    pub unique_failures: usize,
    pub promoted_count: usize,
    pub promoted_dir: String,
    pub findings: Vec<CaseRecord>,
    #[serde(default)]
    pub baseline_stats: LayerStats,
    #[serde(default)]
    pub llm_stats: LayerStats,
    #[serde(default)]
    pub mutation_stats: LayerStats,
    #[serde(default)]
    pub feedback_stats: LayerStats,
}

impl Default for CampaignReport {
    fn default() -> Self {
        Self {
            target_name: String::new(),
            target_kind: String::new(),
            target_entry: String::new(),
            timeout_ms: 2000,
            total_budget: 0,
            total_executions: 0,
            crash_count: 0,
            hang_count: 0,
            panic_count: 0,
            unique_failures: 0,
            promoted_count: 0,
            promoted_dir: String::new(),
            findings: Vec::new(),
            baseline_stats: LayerStats::default(),
            llm_stats: LayerStats::default(),
            mutation_stats: LayerStats::default(),
            feedback_stats: LayerStats::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Oracle {
    pub success_exit_codes: Vec<i32>,
    pub failure_exit_codes: Vec<i32>,
    pub capture_stderr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Expectations {
    pub must_not_panic: bool,
    pub must_not_hang: bool,
    pub must_not_segfault: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Strategy {
    pub styles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FuzzTarget {
    pub name: String,
    pub kind: TargetKind,
    pub entry: PathBuf,
    pub input_mode: InputMode,
    pub timeout_ms: u64,
    pub oracle: Oracle,
    pub expectations: Expectations,
    pub seed_files: Vec<PathBuf>,
    pub strategy: Strategy,
}

impl FuzzTarget {
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.name.is_empty() {
            anyhow::bail!("target name must not be empty");
        }
        if self.timeout_ms == 0 {
            anyhow::bail!("timeout_ms must be greater than zero");
        }
        Ok(())
    }
}
