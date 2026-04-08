use fz_core::{FuzzTarget, TargetKind};

pub fn build_seed_prompt(target: &FuzzTarget, count: usize) -> String {
    let kind_desc = match target.kind {
        TargetKind::Cli => "a CLI tool that reads text from stdin or command-line arguments",
        TargetKind::Api => "a REST API that accepts text input (JSON, YAML, etc.)",
        TargetKind::Repl => "a REPL (read-eval-print loop) that processes text commands",
    };

    format!(
        "Generate {count} malformed text inputs that might crash {kind_desc} named \"{name}\". \
         Focus on edge cases: empty strings, extremely long input, invalid encodings, \
         special characters, boundary values, deeply nested structures, contradictory syntax. \
         Return each input on a separate line with no numbering or markdown formatting.",
        kind_desc = kind_desc,
        name = target.name,
        count = count,
    )
}

pub fn build_crash_analysis_prompt(target: &FuzzTarget, stderr: &str) -> String {
    format!(
        "Given this crash output from the \"{name}\" program, suggest a minimized input \
         that reproduces the same crash. Return only the minimized input, nothing else.\n\n\
         stderr:\n{stderr}",
        name = target.name,
        stderr = stderr,
    )
}

pub fn build_grammar_prompt(target: &FuzzTarget, examples: &[String]) -> String {
    let examples_text = examples
        .iter()
        .take(5)
        .enumerate()
        .map(|(i, ex)| format!("{i}. {ex}"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Given these valid inputs from the \"{name}\" program, infer the likely grammar \
         and suggest 10 edge-case inputs that stress the grammar boundaries.\n\n\
         Valid inputs:\n{examples_text}",
        name = target.name,
        examples_text = examples_text,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::{Expectations, InputMode, Oracle, Strategy};
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
}
