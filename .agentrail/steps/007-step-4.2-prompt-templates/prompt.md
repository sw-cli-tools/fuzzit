Phase 4, Step 4.2: Prompt templates for LLM seed generation

Implement prompt templates in crates/fz-llm/.

Templates for:
1. Edge-case input generation (per target type):
   - Compiler: 'Generate 10 malformed source code snippets that might crash a {language} compiler. Focus on: unterminated strings, deeply nested expressions, invalid escape sequences, contradictory type annotations, extremely long identifiers.'
   - API/JSON: 'Generate 10 malformed JSON inputs that might crash a REST API. Focus on: duplicate keys, conflicting types, extremely long strings, invalid unicode, missing required fields.'
   - REPL: 'Generate 10 command sequences that might crash a REPL. Focus on: mode transitions, undefined variables, extremely long input, control characters, recursive commands.'
   - Config parser: 'Generate 10 malformed config file contents. Focus on: duplicate keys, circular includes, invalid numbers, unterminated strings, mixed encodings.'

2. Crash analysis:
   'Given this crash output from a {target_type} program, suggest a minimized input that reproduces the same crash: {stderr}'

3. Grammar inference:
   'Given these valid inputs from a {target_type} program, infer the likely grammar and suggest 10 edge-case inputs that stress the grammar: {examples}'

Public API:
- fn build_seed_prompt(target: &FuzzTarget, count: usize) -> String
- fn build_crash_analysis_prompt(target: &FuzzTarget, stderr: &str) -> String
- fn build_grammar_prompt(target: &FuzzTarget, examples: &[String]) -> String

TDD tests:
- Each template function produces non-empty string
- Template includes target name
- Template includes target kind-specific content
- No panic on empty/missing fields
- Compiler target gets compiler-specific prompt
- API target gets API-specific prompt