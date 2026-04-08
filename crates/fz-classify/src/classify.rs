use fz_core::{Classification, ExecutionResult, Oracle};

pub fn classify(result: &ExecutionResult, oracle: &Oracle, _timeout_ms: u64) -> Classification {
    if result.killed {
        return Classification::Hang;
    }

    if let Some(code) = result.exit_code {
        if oracle.failure_exit_codes.contains(&code) {
            if is_panic_stderr(&result.stderr) {
                return Classification::Panic;
            }
            if is_segfault_code(code) {
                return Classification::Segfault;
            }
            return Classification::UnexpectedExit;
        }
        if oracle.success_exit_codes.contains(&code) {
            if !result.stderr.is_empty() {
                return Classification::UnexpectedStderr;
            }
            return Classification::Success;
        }
        return Classification::UnexpectedExit;
    }

    Classification::Success
}

fn is_panic_stderr(stderr: &str) -> bool {
    stderr.contains("thread '") && stderr.contains("panicked")
}

fn is_segfault_code(code: i32) -> bool {
    matches!(code, 134 | 137 | 139)
}

pub fn signature(result: &ExecutionResult) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    result.exit_code.hash(&mut hasher);
    let stderr_key = truncate_stderr(result.stderr.as_bytes(), 256);
    hasher.write(&stderr_key);
    hasher.finish()
}

fn truncate_stderr(data: &[u8], max_len: usize) -> Vec<u8> {
    if data.len() <= max_len {
        data.to_vec()
    } else {
        data[..max_len].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fz_core::Oracle;

    fn default_oracle() -> Oracle {
        Oracle {
            success_exit_codes: vec![0],
            failure_exit_codes: vec![1, 101, 134, 137, 139],
            capture_stderr: true,
        }
    }

    fn success_result() -> ExecutionResult {
        ExecutionResult {
            exit_code: Some(0),
            stdout: vec![],
            stderr: String::new(),
            wall_time_ms: 10,
            killed: false,
        }
    }

    #[test]
    fn success_classification() {
        let r = success_result();
        assert_eq!(
            classify(&r, &default_oracle(), 2000),
            Classification::Success
        );
    }

    #[test]
    fn panic_classification() {
        let mut r = success_result();
        r.exit_code = Some(101);
        r.stderr = "thread 'main' panicked at 'test'".into();
        assert_eq!(classify(&r, &default_oracle(), 2000), Classification::Panic);
    }

    #[test]
    fn hang_classification() {
        let r = ExecutionResult {
            exit_code: None,
            stdout: vec![],
            stderr: String::new(),
            wall_time_ms: 2000,
            killed: true,
        };
        assert_eq!(classify(&r, &default_oracle(), 2000), Classification::Hang);
    }

    #[test]
    fn segfault_134() {
        let mut r = success_result();
        r.exit_code = Some(134);
        assert_eq!(
            classify(&r, &default_oracle(), 2000),
            Classification::Segfault
        );
    }

    #[test]
    fn segfault_139() {
        let mut r = success_result();
        r.exit_code = Some(139);
        assert_eq!(
            classify(&r, &default_oracle(), 2000),
            Classification::Segfault
        );
    }

    #[test]
    fn unexpected_exit() {
        let mut r = success_result();
        r.exit_code = Some(42);
        assert_eq!(
            classify(&r, &default_oracle(), 2000),
            Classification::UnexpectedExit
        );
    }

    #[test]
    fn unexpected_stderr() {
        let mut r = success_result();
        r.stderr = "warning: something".into();
        assert_eq!(
            classify(&r, &default_oracle(), 2000),
            Classification::UnexpectedStderr
        );
    }

    #[test]
    fn signature_is_deterministic() {
        let r = success_result();
        assert_eq!(signature(&r), signature(&r));
    }

    #[test]
    fn signature_differs_for_different_stderr() {
        let mut r1 = success_result();
        let mut r2 = success_result();
        r1.stderr = "error A".into();
        r2.stderr = "error B".into();
        assert_ne!(signature(&r1), signature(&r2));
    }

    #[test]
    fn signature_differs_for_different_exit_code() {
        let mut r1 = success_result();
        let mut r2 = success_result();
        r1.exit_code = Some(0);
        r2.exit_code = Some(1);
        assert_ne!(signature(&r1), signature(&r2));
    }
}
