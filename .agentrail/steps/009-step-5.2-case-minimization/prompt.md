Phase 5, Step 5.2: Case minimization

Implement crash case minimization.

Goal: Given a crashing input, find the smallest input that still produces the same failure.

Public API (in fz-mutate or new fz-minimize crate if needed):
- fn minimize(target: &FuzzTarget, input: &[u8], max_iterations: usize) -> anyhow::Result<Vec<u8>>

Algorithm:
1. Execute original input, record classification and stderr signature
2. Binary search: try removing first half, then second half
3. For each half-removal, if it still crashes with same signature, keep the smaller input
4. Also try byte-level removals: remove individual bytes
5. Repeat until no further reduction possible or max_iterations reached
6. Return smallest crashing input found

TDD tests:
- Minimized input is smaller than or equal to original
- Minimized input still produces same classification
- Empty crasher returns empty (edge case)
- Single-byte crasher returns itself
- Large input is reduced significantly
- max_iterations is respected (returns best found so far)
- Non-crashing input returns error or original