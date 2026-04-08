use fz_classify::{classify, signature};
use fz_core::{Classification, FuzzTarget};

pub fn minimize(
    target: &FuzzTarget,
    input: &[u8],
    max_iterations: usize,
) -> anyhow::Result<Vec<u8>> {
    let original_result = fz_exec::execute(target, input)?;
    let original_sig = signature(&original_result);

    if matches!(
        classify(&original_result, &target.oracle, target.timeout_ms),
        Classification::Success | Classification::UnexpectedStderr
    ) {
        return Ok(input.to_vec());
    }

    let mut best = input.to_vec();
    let mut iterations = 0usize;

    while iterations < max_iterations {
        iterations += 1;
        let prev_len = best.len();

        if best.len() <= 1 {
            break;
        }

        let mut improved = false;

        let half = best.len() / 2;

        for (start, end) in [(half, best.len()), (0, best.len() - half)] {
            if start >= end || start >= best.len() {
                continue;
            }
            let mut candidate = best[..start].to_vec();
            candidate.extend_from_slice(&best[end..]);

            let result = match fz_exec::execute(target, &candidate) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if signature(&result) == original_sig {
                best = candidate;
                improved = true;
                break;
            }
        }

        if !improved && best.len() > 1 {
            let idx = best.len() / 2;
            let mut candidate = best[..idx].to_vec();
            candidate.extend_from_slice(&best[idx + 1..]);

            let result = match fz_exec::execute(target, &candidate) {
                Ok(r) => r,
                Err(_) => continue,
            };

            if signature(&result) == original_sig {
                best = candidate;
            }
        }

        if best.len() >= prev_len {
            break;
        }
    }

    Ok(best)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::{Expectations, InputMode, Oracle, Strategy, TargetKind};
    use std::io::Write;
    use std::path::PathBuf;

    fn make_crasher(dir: &tempfile::TempDir, exit_code: i32) -> PathBuf {
        let path = dir.path().join("crasher");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "#!/bin/sh").unwrap();
        writeln!(f, "exit {exit_code}").unwrap();
        std::process::Command::new("chmod")
            .args(["+x", path.to_str().unwrap()])
            .status()
            .unwrap();
        path
    }

    fn make_target(entry: PathBuf) -> FuzzTarget {
        FuzzTarget {
            name: "test".into(),
            kind: TargetKind::Cli,
            entry,
            input_mode: InputMode::Stdin,
            timeout_ms: 5000,
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
    fn minimized_is_smaller_or_equal() {
        let tmp = tempfile::tempdir().unwrap();
        let target = make_target(make_crasher(&tmp, 1));
        let input = b"AAAA BBBB CCCC DDDD EEEE FFFF GGGG HHHH IIII JJJJ";
        let result = minimize(&target, input, 50).unwrap();
        assert!(result.len() <= input.len());
    }

    #[test]
    fn non_crashing_returns_original() {
        let tmp = tempfile::tempdir().unwrap();
        let target = make_target(make_crasher(&tmp, 0));
        let input = b"hello world";
        let result = minimize(&target, input, 10).unwrap();
        assert_eq!(result, input);
    }

    #[test]
    fn single_byte_crasher_returns_itself() {
        let tmp = tempfile::tempdir().unwrap();
        let target = make_target(make_crasher(&tmp, 1));
        let input = b"X";
        let result = minimize(&target, input, 10).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn max_iterations_respected() {
        let tmp = tempfile::tempdir().unwrap();
        let target = make_target(make_crasher(&tmp, 1));
        let input = vec![b'A'; 1000];
        let result = minimize(&target, &input, 1).unwrap();
        assert!(result.len() <= input.len());
    }

    #[test]
    fn empty_input_returns_empty() {
        let tmp = tempfile::tempdir().unwrap();
        let target = make_target(make_crasher(&tmp, 1));
        let result = minimize(&target, b"", 10).unwrap();
        assert!(result.is_empty());
    }
}
