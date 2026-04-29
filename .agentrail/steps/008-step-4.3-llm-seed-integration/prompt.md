Phase 4, Step 4.3: LLM seed integration with campaigns

Wire LLM seed generation into the campaign pipeline as Layer 2.

Changes:
- After baseline (Layer 1), before mutation (Layer 3), call LLM for seeds
- Build prompt using fz_llm templates
- Call Ollama, parse response as list of input strings
- Validate each LLM-generated input (non-empty, reasonable length)
- Execute validated inputs, classify, track provenance as Llm
- If Ollama unavailable, skip Layer 2 with a warning (not an error)

Budget allocation (default):
- Layer 1 (baseline): 30%
- Layer 2 (LLM seeds): 10%
- Layer 3 (mutation): 40%
- Layer 4 (feedback): 20%

Configurable via --layer-budgets flag or manifest [strategy] section.

TDD tests:
- With mocked LLM, campaign includes Llm-provenance cases
- Without Ollama, campaign skips LLM layer and logs warning
- LLM-generated inputs that are empty are filtered out
- LLM-generated inputs that are too long (>1MB) are filtered out
- Budget allocation is respected
- Campaign report includes Layer 2 stats

Update fz-cli campaigns start to include all 4 layers.