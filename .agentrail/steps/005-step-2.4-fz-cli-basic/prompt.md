Phase 2, Step 2.4: fz-cli basic CLI skeleton

Implement the basic CLI entrypoint at crates/fz-cli/.

Subcommands:
- fuzzit targets run --manifest <path>
  Load manifest, generate baseline corpus, execute each input, classify, write report.
- fuzzit campaigns report --dir <path>
  Load report.json from dir, print summary to stdout.

Wire up the pipeline: manifest -> corpus -> exec -> classify -> artifacts.

Implementation:
- Use clap with derive API
- Main function dispatches to subcommand handlers
- Each handler is in its own module (commands/ directory)
- Keep main.rs minimal (just parse args and dispatch)

For 'targets run':
1. Parse manifest with fz_manifest::parse_manifest
2. Generate baseline corpus with fz_corpus::generate_baseline_corpus
3. For each corpus entry, execute with fz_exec::execute
4. Classify each result with fz_classify::classify
5. Collect into CampaignReport
6. Write artifacts with fz_artifacts::write_report

For 'campaigns report':
1. Read report.json from dir
2. Print summary to stdout

TDD tests:
- Parse CLI args correctly
- 'targets run' with valid manifest produces artifacts directory
- 'campaigns report' with existing report prints summary
- Invalid manifest path returns clear error
- Missing --manifest flag shows help

Dependencies: fz-core, fz-manifest, fz-corpus, fz-exec, fz-classify, fz-artifacts, clap, anyhow.