pub mod client;
pub mod prompts;

pub use client::OllamaClient;
pub use prompts::{build_crash_analysis_prompt, build_grammar_prompt, build_seed_prompt};
