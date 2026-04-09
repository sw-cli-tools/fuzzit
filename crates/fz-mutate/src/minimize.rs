use fz_classify::{classify, signature};
use fz_core::{Classification, FuzzTarget};

fn try_remove_range(
    target: &FuzzTarget,
    input: &[u8],
    start: usize,
    end: usize,
    original_sig: u64,
) -> Option<Vec<u8>> {
    if start >= end || start >= input.len() {
        return None;
    }
    let mut candidate = input[..start].to_vec();
    candidate.extend_from_slice(&input[end..]);

    let result = fz_exec::execute(target, &candidate).ok()?;
    if signature(&result) == original_sig {
        Some(candidate)
    } else {
        None
    }
}

fn is_non_crash(result: &fz_core::ExecutionResult, target: &FuzzTarget) -> bool {
    matches!(
        classify(result, &target.oracle, target.timeout_ms),
        Classification::Success | Classification::UnexpectedStderr
    )
}

pub fn minimize(
    target: &FuzzTarget,
    input: &[u8],
    max_iterations: usize,
) -> anyhow::Result<Vec<u8>> {
    let original_result = fz_exec::execute(target, input)?;
    if is_non_crash(&original_result, target) {
        return Ok(input.to_vec());
    }
    let original_sig = signature(&original_result);

    let mut best = input.to_vec();
    for _ in 0..max_iterations {
        if best.len() <= 1 {
            break;
        }
        let prev_len = best.len();
        let half = best.len() / 2;

        let improved = try_remove_range(target, &best, half, best.len(), original_sig)
            .or_else(|| try_remove_range(target, &best, 0, best.len() - half, original_sig));

        if let Some(smaller) = improved {
            best = smaller;
        } else if best.len() > 1 {
            let idx = best.len() / 2;
            if let Some(smaller) = try_remove_range(target, &best, idx, idx + 1, original_sig) {
                best = smaller;
            }
        }

        if best.len() >= prev_len {
            break;
        }
    }

    Ok(best)
}
