use std::path::Path;

use anyhow::Context;
use fz_core::{Expectations, FuzzTarget, InputMode, Oracle, Strategy, TargetKind};

#[derive(serde::Deserialize)]
struct Manifest {
    target: TargetSection,
    oracle: OracleSection,
    expectations: ExpectationsSection,
    seeds: Option<SeedsSection>,
    strategy: Option<StrategySection>,
}

#[derive(serde::Deserialize)]
struct TargetSection {
    name: String,
    kind: TargetKind,
    entry: String,
    input_mode: InputMode,
    timeout_ms: u64,
}

#[derive(serde::Deserialize)]
struct OracleSection {
    success_exit_codes: Option<Vec<i32>>,
    failure_exit_codes: Option<Vec<i32>>,
    capture_stderr: Option<bool>,
}

#[derive(serde::Deserialize)]
struct ExpectationsSection {
    must_not_panic: Option<bool>,
    must_not_hang: Option<bool>,
    must_not_segfault: Option<bool>,
}

#[derive(serde::Deserialize)]
struct SeedsSection {
    files: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct StrategySection {
    styles: Option<Vec<String>>,
}

pub fn parse_manifest(path: &Path) -> anyhow::Result<FuzzTarget> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read manifest file: {}", path.display()))?;

    if content.trim().is_empty() {
        anyhow::bail!("manifest file is empty: {}", path.display());
    }

    let manifest: Manifest = toml::from_str(&content)
        .with_context(|| format!("failed to parse manifest TOML: {}", path.display()))?;

    let target = FuzzTarget {
        name: manifest.target.name,
        kind: manifest.target.kind,
        entry: std::path::PathBuf::from(&manifest.target.entry),
        input_mode: manifest.target.input_mode,
        timeout_ms: manifest.target.timeout_ms,
        oracle: Oracle {
            success_exit_codes: manifest.oracle.success_exit_codes.unwrap_or_default(),
            failure_exit_codes: manifest.oracle.failure_exit_codes.unwrap_or_default(),
            capture_stderr: manifest.oracle.capture_stderr.unwrap_or(true),
        },
        expectations: Expectations {
            must_not_panic: manifest.expectations.must_not_panic.unwrap_or(true),
            must_not_hang: manifest.expectations.must_not_hang.unwrap_or(true),
            must_not_segfault: manifest.expectations.must_not_segfault.unwrap_or(true),
        },
        seed_files: manifest
            .seeds
            .and_then(|s| s.files)
            .unwrap_or_default()
            .into_iter()
            .map(std::path::PathBuf::from)
            .collect(),
        strategy: Strategy {
            styles: manifest.strategy.and_then(|s| s.styles).unwrap_or_default(),
        },
    };

    target.validate()?;

    Ok(target)
}
