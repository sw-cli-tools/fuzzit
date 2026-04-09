use std::time::Duration;

use anyhow::{Context, bail};
use serde::{Deserialize, Serialize};

const DEFAULT_BASE_URL: &str = "http://localhost:11434";
const DEFAULT_TIMEOUT_SECS: u64 = 60;
const MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone)]
pub struct OllamaClient {
    base_url: String,
    model: String,
    timeout: Duration,
}

#[derive(Debug, Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct GenerateResponse {
    response: String,
}

impl OllamaClient {
    pub fn new(model: &str) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
            model: model.into(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn with_options(model: &str, base_url: &str, timeout: Duration) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            timeout,
        }
    }

    pub fn is_available(&self) -> bool {
        let url = format!("{}/api/tags", self.base_url);
        let client = reqwest::blocking::Client::builder()
            .timeout(self.timeout)
            .build();
        let client = match client {
            Ok(c) => c,
            Err(_) => return false,
        };
        client.get(&url).send().is_ok()
    }

    pub fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        let body = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.into(),
            stream: false,
        };

        let client = reqwest::blocking::Client::builder()
            .timeout(self.timeout)
            .build()
            .context("failed to create HTTP client")?;

        let mut last_err = None;
        for attempt in 0..MAX_RETRIES {
            let result = client.post(&url).json(&body).send();
            match result {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        bail!("Ollama returned status {}", resp.status());
                    }
                    let resp_body: GenerateResponse =
                        resp.json().context("failed to parse Ollama response")?;
                    return Ok(resp_body.response);
                }
                Err(e) => {
                    last_err = Some(e);
                    if attempt < MAX_RETRIES - 1 {
                        let delay = Duration::from_secs(2u64.pow(attempt));
                        std::thread::sleep(delay);
                    }
                }
            }
        }
        bail!(
            "Ollama request failed after {} retries: {}",
            MAX_RETRIES,
            last_err.unwrap()
        );
    }
}
