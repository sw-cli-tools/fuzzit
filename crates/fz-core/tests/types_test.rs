use fz_core::*;

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
        entry: std::path::PathBuf::from("/bin/true"),
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
        entry: std::path::PathBuf::from("/bin/true"),
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
        entry: std::path::PathBuf::from("/bin/true"),
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
        entry: std::path::PathBuf::from("/bin/true"),
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
        seed_files: vec![std::path::PathBuf::from("seeds/a.txt")],
        strategy: Strategy {
            styles: vec!["grammarish".into()],
        },
    };
    let json = serde_json::to_string(&target).unwrap();
    let back: FuzzTarget = serde_json::from_str(&json).unwrap();
    assert_eq!(target, back);
}
