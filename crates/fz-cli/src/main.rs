mod commands;

use clap::{Parser, Subcommand};

const VERSION: &str = "0.1.0
Copyright (c) 2026 Softwarewrighter
License: Private
Repository: https://github.com/anomalyco/sw-cli-tools
Build Host: unknown
Build Commit: unknown
Build Time: unknown";

#[derive(Parser)]
#[command(
    name = "fuzzit",
    version = VERSION,
    long_version = VERSION,
    about = "LLM-guided fuzz testing tool",
    long_about = "\
LLM-guided fuzz testing tool for CLI programs, compilers, interpreters, REPLs, and APIs.

Combines deterministic baseline fuzzing, AI-generated edge cases via Ollama,
mutation engines, and feedback loops to discover crashes, hangs, panics,
and unexpected behavior in text-input programs.

AI CODING AGENT INSTRUCTIONS:
When working with this codebase, follow these rules:
- This is a Rust project using Edition 2024
- Run `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` before committing
- Never use #[allow(...)] to suppress clippy warnings -- fix the underlying issue
- The project uses a 9-crate workspace layout under crates/
- See AGENTS.md for the full development workflow and agentrail session protocol
- TDD required: write failing tests first, then implement
- File size limit: 500 lines, function size limit: 50 lines, max 7 functions per module
"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run fuzz targets against a manifest
    Targets {
        /// Subcommand
        #[command(subcommand)]
        action: TargetsAction,
    },
    /// Manage fuzz campaigns
    Campaigns {
        /// Subcommand
        #[command(subcommand)]
        action: CampaignsAction,
    },
}

#[derive(Subcommand)]
enum TargetsAction {
    /// Execute a fuzz target with baseline corpus
    Run {
        /// Path to target manifest (TOML)
        #[arg(long)]
        manifest: String,
    },
    /// Generate seeds for a fuzz target
    Generate {
        /// Path to target manifest (TOML)
        #[arg(long)]
        manifest: String,
        /// Number of seeds to generate
        #[arg(long, default_value = "100")]
        budget: usize,
    },
}

#[derive(Subcommand)]
enum CampaignsAction {
    /// Start a multi-layer fuzz campaign
    Start {
        /// Path to target manifest (TOML)
        #[arg(long)]
        manifest: String,
        /// Total execution budget
        #[arg(long, default_value = "500")]
        budget: usize,
    },
    /// Display a campaign report
    Report {
        /// Path to campaign artifacts directory
        #[arg(long)]
        dir: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Targets { action } => match action {
            TargetsAction::Run { manifest } => {
                let path = std::path::Path::new(&manifest);
                commands::run_targets(path)?;
            }
            TargetsAction::Generate {
                manifest: _,
                budget: _,
            } => {
                anyhow::bail!("targets generate not yet implemented");
            }
        },
        Commands::Campaigns { action } => match action {
            CampaignsAction::Start { manifest, budget } => {
                let path = std::path::Path::new(&manifest);
                commands::start_campaign(path, budget, "llama3")?;
            }
            CampaignsAction::Report { dir } => {
                let dir = std::path::Path::new(&dir);
                let report_path = dir.join("report.json");
                if !report_path.exists() {
                    anyhow::bail!("report not found: {}", report_path.display());
                }
                let content = std::fs::read_to_string(&report_path)?;
                let report: fz_core::CampaignReport = serde_json::from_str(&content)?;
                println!("Target: {}", report.target_name);
                println!("Executions: {}", report.total_executions);
                println!("Panics: {}", report.panic_count);
                println!("Hangs: {}", report.hang_count);
                println!("Crashes: {}", report.crash_count);
                println!("Unique failures: {}", report.unique_failures);
            }
        },
    }

    Ok(())
}
