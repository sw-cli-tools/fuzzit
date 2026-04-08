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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_kind_roundtrip() {
        let kinds = [TargetKind::Cli, TargetKind::Api, TargetKind::Repl];
        for kind in &kinds {
            let serialized = serde_json::to_string(kind).unwrap();
            let deserialized: TargetKind = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*kind, deserialized);
        }
    }

    #[test]
    fn input_mode_roundtrip() {
        let modes = [InputMode::Stdin, InputMode::Args, InputMode::File];
        for mode in &modes {
            let serialized = serde_json::to_string(mode).unwrap();
            let deserialized: InputMode = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*mode, deserialized);
        }
    }

    #[test]
    fn provenance_values() {
        let all = [
            Provenance::Baseline,
            Provenance::Llm,
            Provenance::Mutation,
            Provenance::Feedback,
            Provenance::UserSeed,
        ];
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn classification_values() {
        let all = [
            Classification::Success,
            Classification::Panic,
            Classification::Hang,
            Classification::Segfault,
            Classification::UnexpectedExit,
            Classification::UnexpectedStderr,
        ];
        assert_eq!(all.len(), 6);
    }

    #[test]
    fn validate_rejects_empty_name() {
        let target = FuzzTarget {
            name: String::new(),
            kind: TargetKind::Cli,
            entry: PathBuf::from("/bin/true"),
            input_mode: InputMode::Stdin,
            timeout_ms: 2000,
            oracle: Oracle {
                success_exit_codes: vec![0],
                failure_exit_codes: vec![1],
                capture_stderr: true,
            },
            expectations: Expectations {
                must_not_panic: true,
                must_not_hang: true,
                must_not_segfault: true,
            },
            seed_files: vec![],
            strategy: Strategy { styles: vec![] },
        };
        let result = target.validate();
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("name"));
    }

    #[test]
    fn validate_rejects_zero_timeout() {
        let target = FuzzTarget {
            name: "test".into(),
            kind: TargetKind::Cli,
            entry: PathBuf::from("/bin/true"),
            input_mode: InputMode::Stdin,
            timeout_ms: 0,
            oracle: Oracle {
                success_exit_codes: vec![0],
                failure_exit_codes: vec![1],
                capture_stderr: true,
            },
            expectations: Expectations {
                must_not_panic: true,
                must_not_hang: true,
                must_not_segfault: true,
            },
            seed_files: vec![],
            strategy: Strategy { styles: vec![] },
        };
        let result = target.validate();
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn validate_accepts_valid_target() {
        let target = FuzzTarget {
            name: "test".into(),
            kind: TargetKind::Cli,
            entry: PathBuf::from("/bin/true"),
            input_mode: InputMode::Stdin,
            timeout_ms: 2000,
            oracle: Oracle {
                success_exit_codes: vec![0],
                failure_exit_codes: vec![1],
                capture_stderr: true,
            },
            expectations: Expectations {
                must_not_panic: true,
                must_not_hang: true,
                must_not_segfault: true,
            },
            seed_files: vec![],
            strategy: Strategy { styles: vec![] },
        };
        assert!(target.validate().is_ok());
    }

    #[test]
    fn fuzz_target_json_roundtrip() {
        let target = FuzzTarget {
            name: "test".into(),
            kind: TargetKind::Cli,
            entry: PathBuf::from("/bin/true"),
            input_mode: InputMode::Stdin,
            timeout_ms: 2000,
            oracle: Oracle {
                success_exit_codes: vec![0],
                failure_exit_codes: vec![1],
                capture_stderr: true,
            },
            expectations: Expectations {
                must_not_panic: true,
                must_not_hang: true,
                must_not_segfault: true,
            },
            seed_files: vec![PathBuf::from("seeds/a.txt")],
            strategy: Strategy {
                styles: vec!["grammarish".into()],
            },
        };
        let json = serde_json::to_string(&target).unwrap();
        let back: FuzzTarget = serde_json::from_str(&json).unwrap();
        assert_eq!(target, back);
    }
}
