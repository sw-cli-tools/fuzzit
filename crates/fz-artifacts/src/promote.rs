use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use fz_core::{CaseRecord, FuzzTarget};

use super::promote_helpers::{
    expected_behavior, format_input_literal, format_input_preview, sanitize_test_name,
};

fn generate_test_content(target: &FuzzTarget, case: &CaseRecord, index: usize) -> String {
    let test_name = sanitize_test_name(&target.name, index);
    let input_literal = format_input_literal(&case.input);
    let classification_str = format!("{:?}", case.classification);
    let provenance_str = format!("{:?}", case.provenance);
    let timestamp = if case.discovered_at.is_empty() {
        "unknown".to_string()
    } else {
        case.discovered_at.clone()
    };
    let preview = format_input_preview(&case.input, 80);
    let expected = expected_behavior(&case.classification);

    format!(
        "// Auto-generated fuzzit regression test\n\
         // Target: {name}\n\
         // Classification: {classification}\n\
         // Provenance: {provenance}\n\
         // Discovered: {timestamp}\n\
         // Input preview: {preview}\n\n\
         #[test]\n\
         fn {test_name}() {{\n\
     let input = {input_literal};\n\
     // This test documents a fuzzit finding.\n\
     // Wire up actual target invocation to make it a real regression test.\n\
     // Expected: should not {expected}\n\
     // TODO: replace this panic with actual target invocation.\n\
     panic!(\"fuzzit regression placeholder: run `fuzzit` to reproduce\");\n\
 }}\n",
        name = target.name,
        classification = classification_str,
        provenance = provenance_str,
        timestamp = timestamp,
        preview = preview,
        test_name = test_name,
        input_literal = input_literal,
        expected = expected,
    )
}

fn write_test_file(path: &Path, test_name: &str, content: &str) -> anyhow::Result<()> {
    if path.exists() {
        let existing = std::fs::read_to_string(path)?;
        if existing.contains(&format!("fn {test_name}()")) {
            return Ok(());
        }
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(path)
            .context("failed to open regression file")?;
        file.write_all(content.as_bytes())?;
    } else {
        std::fs::write(path, content.as_bytes()).context("failed to write regression file")?;
    }
    Ok(())
}

pub fn promote_to_test(
    output_dir: &Path,
    target: &FuzzTarget,
    case: &CaseRecord,
    index: usize,
) -> anyhow::Result<PathBuf> {
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("failed to create output dir: {}", output_dir.display()))?;

    let test_name = sanitize_test_name(&target.name, index);
    let content = generate_test_content(target, case, index);
    let path = output_dir.join("fuzzit_regressions.rs");
    write_test_file(&path, &test_name, &content)?;
    Ok(path)
}

pub fn promote_batch(
    output_dir: &Path,
    target: &FuzzTarget,
    cases: &[CaseRecord],
) -> anyhow::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for (i, case) in cases.iter().enumerate() {
        let path = promote_to_test(output_dir, target, case, i)?;
        paths.push(path);
    }
    Ok(paths)
}
