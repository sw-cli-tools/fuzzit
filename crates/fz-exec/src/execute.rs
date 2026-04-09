use std::io::Write;
use std::process::Stdio;
use std::time::Instant;

use anyhow::Context;
use fz_core::{ExecutionResult, FuzzTarget, InputMode};

fn prepare_input(
    cmd: &mut std::process::Command,
    target: &FuzzTarget,
    input: &[u8],
) -> anyhow::Result<Option<tempfile::NamedTempFile>> {
    match target.input_mode {
        InputMode::Stdin => Ok(None),
        InputMode::Args => {
            let input_str = std::str::from_utf8(input)
                .map_err(|_| anyhow::anyhow!("args input_mode requires valid UTF-8"))?;
            cmd.arg(input_str);
            Ok(None)
        }
        InputMode::File => {
            let temp_file =
                tempfile::NamedTempFile::new().context("failed to create temp file for input")?;
            temp_file
                .as_file()
                .write_all(input)
                .context("failed to write input to temp file")?;
            cmd.arg(temp_file.path());
            Ok(Some(temp_file))
        }
    }
}

pub fn execute(target: &FuzzTarget, input: &[u8]) -> anyhow::Result<ExecutionResult> {
    let mut cmd = std::process::Command::new(&target.entry);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let _temp_file = prepare_input(&mut cmd, target, input)?;

    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn target: {}", target.entry.display()))?;

    if matches!(target.input_mode, InputMode::Stdin)
        && let Some(ref mut stdin) = child.stdin
    {
        stdin
            .write_all(input)
            .context("failed to write to target stdin")?;
    }
    drop(child.stdin.take());

    let start = Instant::now();
    let output = child
        .wait_with_output()
        .context("failed to collect target output")?;
    let wall_time_ms = start.elapsed().as_millis() as u64;

    Ok(ExecutionResult {
        exit_code: output.status.code(),
        stdout: output.stdout,
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        wall_time_ms,
        killed: false,
    })
}
