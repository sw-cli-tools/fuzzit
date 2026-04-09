use fz_core::{Expectations, InputMode, Oracle, Strategy};
use fz_core::{FuzzTarget, TargetKind};
use fz_llm::{OllamaClient, build_crash_analysis_prompt, build_grammar_prompt, build_seed_prompt};
use std::path::PathBuf;

fn sample_target(kind: TargetKind) -> FuzzTarget {
    FuzzTarget {
        name: "test-parser".into(),
        kind,
        entry: PathBuf::from("/bin/test"),
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
    }
}

#[test]
fn client_new_defaults() {
    let client = OllamaClient::new("llama3");
    assert_eq!(client.model(), "llama3");
}

#[test]
fn client_with_options() {
    let client = OllamaClient::with_options(
        "mistral",
        "http://localhost:11435",
        std::time::Duration::from_secs(30),
    );
    assert_eq!(client.model(), "mistral");
}

#[test]
fn seed_prompt_contains_target_name() {
    let target = sample_target(TargetKind::Cli);
    let prompt = build_seed_prompt(&target, 10);
    assert!(prompt.contains("test-parser"));
}

#[test]
fn seed_prompt_contains_count() {
    let target = sample_target(TargetKind::Cli);
    let prompt = build_seed_prompt(&target, 25);
    assert!(prompt.contains("25"));
}

#[test]
fn seed_prompt_cli_specific() {
    let target = sample_target(TargetKind::Cli);
    let prompt = build_seed_prompt(&target, 10);
    assert!(prompt.contains("CLI"));
}

#[test]
fn seed_prompt_api_specific() {
    let target = sample_target(TargetKind::Api);
    let prompt = build_seed_prompt(&target, 10);
    assert!(prompt.contains("API"));
}

#[test]
fn seed_prompt_repl_specific() {
    let target = sample_target(TargetKind::Repl);
    let prompt = build_seed_prompt(&target, 10);
    assert!(prompt.contains("REPL"));
}

#[test]
fn crash_prompt_contains_stderr() {
    let target = sample_target(TargetKind::Cli);
    let prompt = build_crash_analysis_prompt(&target, "thread panicked at line 42");
    assert!(prompt.contains("thread panicked at line 42"));
    assert!(prompt.contains("test-parser"));
}

#[test]
fn grammar_prompt_contains_examples() {
    let target = sample_target(TargetKind::Cli);
    let examples = vec!["fn main() {}".into(), "let x = 1;".into()];
    let prompt = build_grammar_prompt(&target, &examples);
    assert!(prompt.contains("fn main()"));
    assert!(prompt.contains("let x = 1;"));
}

#[test]
fn grammar_prompt_empty_examples() {
    let target = sample_target(TargetKind::Cli);
    let prompt = build_grammar_prompt(&target, &[]);
    assert!(!prompt.is_empty());
}
