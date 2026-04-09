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
