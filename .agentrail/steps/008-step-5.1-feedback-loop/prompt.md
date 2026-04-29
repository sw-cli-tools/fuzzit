Phase 5, Step 5.1: Feedback loop

Implement the feedback loop (Layer 4) in the campaign pipeline.

Responsibilities:
- Track interestingness of each execution result
- Feed interesting cases back into the mutation pool
- Prevent infinite loops (max feedback iterations)
- Combine with mutation engine for systematic exploration

Interestingness criteria:
- New exit code (not seen before in campaign)
- New stderr signature (hash differs from all known)
- Significantly longer runtime (>2x previous for same exit code)
- New stack trace pattern

Implementation:
- Maintain a HashSet<u64> of known signatures during campaign
- After each layer, check which results are interesting
- Add interesting inputs to feedback pool
- Apply mutations to feedback pool (reuse fz-mutate)
- Max 3 feedback iterations (configurable)
- Stop if no new interesting cases found in an iteration

TDD tests:
- New exit code is detected as interesting
- New stderr signature is detected as interesting
- Duplicate signature is NOT interesting
- Feedback loop terminates after max iterations
- Feedback loop terminates early if no new findings
- Campaign report includes feedback iteration stats
- Total budget is respected across all feedback iterations