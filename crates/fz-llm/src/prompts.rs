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
