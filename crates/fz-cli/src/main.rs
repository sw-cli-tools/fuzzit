mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fuzzit", about = "LLM-guided fuzz testing tool", version, long_about = None)]
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
            CampaignsAction::Start {
                manifest: _,
                budget: _,
            } => {
                anyhow::bail!("campaigns start not yet implemented");
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
