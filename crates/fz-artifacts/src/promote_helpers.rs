pub fn sanitize_test_name(target_name: &str, index: usize) -> String {
    let base: String = target_name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    format!("fuzzit_{base}_{index:04}")
}

pub fn format_input_literal(input: &[u8]) -> String {
    match std::str::from_utf8(input) {
        Ok(s) => {
            let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
            format!("\"{escaped}\"")
        }
        Err(_) => {
            let bytes: Vec<String> = input.iter().map(|b| format!("\\x{b:02X}")).collect();
            format!("b\"{}\"", bytes.join(""))
        }
    }
}

pub fn format_input_preview(input: &[u8], max_len: usize) -> String {
    let s = String::from_utf8_lossy(input);
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len])
    }
}

pub fn expected_behavior(classification: &fz_core::Classification) -> &'static str {
    use fz_core::Classification;
    match classification {
        Classification::Success => "fail",
        Classification::Panic => "panic",
        Classification::Hang => "hang",
        Classification::Segfault => "segfault",
        Classification::UnexpectedExit => "fail with unexpected exit code",
        Classification::UnexpectedStderr => "produce unexpected stderr",
    }
}
