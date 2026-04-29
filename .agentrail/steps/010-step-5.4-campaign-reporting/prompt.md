Phase 5, Step 5.4: Full campaign reporting

Complete the campaign reporting system.

Responsibilities:
- Aggregate stats across all 4 layers
- Track budget utilization per layer
- Timeline of discoveries (when each crash/finding was found)
- Summary with actionable recommendations

Report sections (Markdown):
1. Target summary (name, kind, entry point)
2. Campaign configuration (budget, layer allocations, timeout)
3. Layer-by-layer breakdown:
   - Executions count
   - Interesting findings count
   - New signatures discovered
   - Budget utilization
4. Findings catalog:
   - Each unique crash/panic/hang with:
     - Minimized reproducer (if available)
     - Classification
     - Provenance
     - Timestamp discovered
5. Promotion summary:
   - How many cases promoted to regression tests
   - Output directory for promoted tests
6. Recommendations:
   - 'X crashes found, Y promoted to tests'
   - 'Consider increasing budget for Layer Z'

Public API:
- fn write_full_report(dir: &Path, report: &CampaignReport) -> anyhow::Result<()>

TDD tests:
- Markdown report contains all expected sections
- JSON report contains all CampaignReport fields
- Report accurately reflects campaign results
- Empty campaign (no findings) produces valid report
- Campaign with all layer types produces correct per-layer stats