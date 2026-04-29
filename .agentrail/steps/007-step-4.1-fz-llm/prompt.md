Phase 4, Step 4.1: fz-llm Ollama client

Implement the Ollama LLM client crate at crates/fz-llm/.

Responsibilities:
- HTTP client for Ollama /api/generate endpoint
- JSON schema enforcement on responses
- Retry with exponential backoff
- Timeout handling
- Graceful fallback when Ollama is unavailable

Public API:
- struct OllamaClient { base_url: String, model: String, timeout: Duration }
- impl OllamaClient {
    fn new(model: &str) -> Self  // defaults to http://localhost:11434
    fn generate(&self, prompt: &str, schema: Option<Value>) -> anyhow::Result<String>
    fn is_available(&self) -> bool
  }

Implementation:
- Use reqwest for HTTP
- POST to /api/generate with { model, prompt, stream: false, format: schema }
- Retry up to 3 times with exponential backoff (1s, 2s, 4s)
- If Ollama unavailable, return error (caller decides fallback)
- is_available does a GET to /api/tags and checks response

TDD tests:
- Mock HTTP server returns valid response, client parses correctly
- Mock HTTP server returns invalid JSON, client returns clear error
- Mock HTTP server is unreachable, client returns clear error after retries
- is_available returns true when mock server is up
- is_available returns false when mock server is down
- Timeout is enforced (mock server that hangs)
- Schema enforcement: response matches schema or error returned

Dependencies: reqwest, serde, serde_json, anyhow, tokio (for async if needed, or use blocking).