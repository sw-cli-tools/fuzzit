Phase 3, Step 3.2: Mutation integration with campaigns

Integrate fz-mutate into the campaign pipeline.

Changes needed:
- Add mutation layer to 'fuzzit campaigns start' subcommand
- After baseline execution, take surviving (non-crash) cases and mutate them
- Execute mutated inputs, classify results
- Track provenance as Mutation
- Configurable mutation budget (e.g., --mutation-budget 100)
- Retain interesting mutations (new exit code, new stderr signature)

Wire up:
1. Run baseline corpus (Layer 1)
2. Collect non-crash cases into mutation pool
3. For each case in pool, generate N mutations (based on budget)
4. Execute each mutation
5. Classify and collect results
6. Write updated campaign report

TDD tests:
- Campaign with mutation enabled produces more executions than baseline only
- Mutation budget is respected (approximately)
- Mutated cases are classified correctly
- Provenance is set to Mutation
- Report includes mutation layer stats

Update docs/architecture.md if pipeline diagram changes.
Update fz-cli with new 'campaigns start' subcommand.