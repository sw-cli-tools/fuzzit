Phase 1, Step 1.3: fz-manifest TOML parsing

Implement the manifest parsing crate at crates/fz-manifest/.

Responsibilities:
- Parse a TOML file into a fz_core::FuzzTarget struct
- Validate all required fields are present and valid
- Provide useful error messages for missing/invalid fields
- Support all manifest sections: [target], [oracle], [expectations], [seeds], [strategy]

Public API:
- fn parse_manifest(path: &Path) -> anyhow::Result<FuzzTarget>
- fn validate_manifest(target: &FuzzTarget) -> anyhow::Result<()>

TDD tests:
- Parse valid manifest, verify all fields populated correctly
- Missing [target] section returns clear error
- Invalid kind value returns clear error
- Missing required field returns clear error naming the field
- Round-trip: parse -> serialize -> parse produces same result
- Empty file returns clear error
- File not found returns clear error

Example manifest for testing:


Dependencies: fz-core, serde, toml, anyhow.