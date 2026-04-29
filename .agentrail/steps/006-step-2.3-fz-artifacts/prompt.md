Phase 2, Step 2.3: fz-artifacts report generation

Implement the artifact/report generation crate at crates/fz-artifacts/.

Responsibilities:
- Create output directory structure
- Write JSON reports (machine-readable)
- Write Markdown reports (human-readable)
- Write individual case files
- Write corpus seed files

Directory structure:


Public API:
- fn init_output_dir(base: &Path) -> anyhow::Result<PathBuf>
- fn write_report(dir: &Path, report: &CampaignReport) -> anyhow::Result<()>
- fn write_case(dir: &Path, index: usize, input: &[u8], record: &CaseRecord) -> anyhow::Result<()>
- fn write_corpus_seed(dir: &Path, index: usize, input: &[u8]) -> anyhow::Result<()>

JSON report format: serialize CampaignReport with serde_json.
Markdown report: target name, execution count, crash/panic/hang counts,
list of interesting findings with truncated input preview.

TDD tests:
- init_output_dir creates directory structure
- write_report produces valid JSON matching CampaignReport
- write_case creates file with input content
- write_corpus_seed creates file in corpus/ subdirectory
- Markdown report contains expected sections
- Non-existent parent directory is created automatically

Dependencies: fz-core, anyhow, serde_json, chrono (for timestamps).