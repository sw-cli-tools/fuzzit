use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TargetKind {
    Cli,
    Api,
    Repl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
}
