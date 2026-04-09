use std::io::Write;
use std::path::Path;

use fz_core::*;
use fz_manifest::parse_manifest;

fn write_temp_manifest(content: &str) -> tempfile::NamedTempFile {
    let mut file = tempfile::NamedTempFile::new().unwrap();
    write!(file, "{content}").unwrap();
    file
}

fn valid_manifest_toml() -> &'static str {
    r#"
[target]
name = "test-lexer"
kind = "cli"
entry = "./target/debug/test-lexer"
input_mode = "stdin"
timeout_ms = 2000

[oracle]
success_exit_codes = [0]
failure_exit_codes = [101, 134]
capture_stderr = true

[expectations]
must_not_panic = true
must_not_hang = true
must_not_segfault = true

[seeds]
files = ["seeds/valid.txt"]

[strategy]
styles = ["grammarish", "mutation"]
"#
}

#[test]
fn parse_valid_manifest() {
    let file = write_temp_manifest(valid_manifest_toml());
    let target = parse_manifest(file.path()).unwrap();

    assert_eq!(target.name, "test-lexer");
    assert_eq!(target.kind, TargetKind::Cli);
    assert_eq!(
        target.entry,
        std::path::PathBuf::from("./target/debug/test-lexer")
    );
    assert_eq!(target.input_mode, InputMode::Stdin);
    assert_eq!(target.timeout_ms, 2000);
    assert_eq!(target.oracle.success_exit_codes, vec![0]);
    assert_eq!(target.oracle.failure_exit_codes, vec![101, 134]);
    assert!(target.oracle.capture_stderr);
    assert!(target.expectations.must_not_panic);
    assert!(target.expectations.must_not_hang);
    assert!(target.expectations.must_not_segfault);
    assert_eq!(
        target.seed_files,
        vec![std::path::PathBuf::from("seeds/valid.txt")]
    );
    assert_eq!(target.strategy.styles, vec!["grammarish", "mutation"]);
}

#[test]
fn parse_manifest_with_defaults() {
    let toml = r#"
[target]
name = "minimal"
kind = "api"
entry = "./bin/api"
input_mode = "args"
timeout_ms = 5000

[oracle]

[expectations]
"#;
    let file = write_temp_manifest(toml);
    let target = parse_manifest(file.path()).unwrap();

    assert_eq!(target.name, "minimal");
    assert!(target.oracle.success_exit_codes.is_empty());
    assert!(target.oracle.capture_stderr);
    assert!(target.expectations.must_not_panic);
    assert!(target.seed_files.is_empty());
    assert!(target.strategy.styles.is_empty());
}

#[test]
fn missing_target_section_errors() {
    let toml = r#"
[oracle]
success_exit_codes = [0]
"#;
    let file = write_temp_manifest(toml);
    let result = parse_manifest(file.path());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("parse") || msg.contains("missing field"),
        "got: {msg}"
    );
}

#[test]
fn invalid_kind_value_errors() {
    let toml = r#"
[target]
name = "bad"
kind = "invalid_kind"
entry = "./bin/test"
input_mode = "stdin"
timeout_ms = 1000
"#;
    let file = write_temp_manifest(toml);
    let result = parse_manifest(file.path());
    assert!(result.is_err());
}

#[test]
fn empty_name_errors() {
    let toml = r#"
[target]
name = ""
kind = "cli"
entry = "./bin/test"
input_mode = "stdin"
timeout_ms = 1000

[oracle]

[expectations]
"#;
    let file = write_temp_manifest(toml);
    let result = parse_manifest(file.path());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("name"));
}

#[test]
fn zero_timeout_errors() {
    let toml = r#"
[target]
name = "test"
kind = "cli"
entry = "./bin/test"
input_mode = "stdin"
timeout_ms = 0

[oracle]

[expectations]
"#;
    let file = write_temp_manifest(toml);
    let result = parse_manifest(file.path());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("timeout"));
}

#[test]
fn file_not_found_errors() {
    let result = parse_manifest(Path::new("/nonexistent/path/manifest.toml"));
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("read"));
}

#[test]
fn invalid_toml_errors() {
    let file = write_temp_manifest("this is not valid toml {{{");
    let result = parse_manifest(file.path());
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("parse") || msg.contains("TOML"), "got: {msg}");
}
