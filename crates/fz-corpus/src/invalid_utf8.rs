use crate::CaseInput;

pub fn invalid_utf8_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(vec![0x80], "single continuation byte"),
        CaseInput::new(vec![0xC0, 0x80], "overlong 2-byte encoding"),
        CaseInput::new(vec![0xE0, 0x80, 0x80], "overlong 3-byte encoding"),
        CaseInput::new(vec![0xF0, 0x80, 0x80, 0x80], "overlong 4-byte encoding"),
        CaseInput::new(vec![0xC2], "truncated 2-byte sequence"),
        CaseInput::new(vec![0xE0, 0xA0], "truncated 3-byte sequence"),
        CaseInput::new(vec![0xF4, 0x90, 0x80, 0x80], "codepoint above U+10FFFF"),
        CaseInput::new(vec![0xED, 0xA0, 0x80], "UTF-16 surrogate (U+D800)"),
        CaseInput::new(
            vec![0xF8, 0x80, 0x80, 0x80, 0x80],
            "invalid 5-byte sequence",
        ),
        CaseInput::new(vec![0xFF, 0xFE], "BOM in wrong order"),
        CaseInput::new(vec![0xFE, 0xFF], "UTF-16 BE BOM"),
        CaseInput::new(vec![0xFF], "invalid single byte 0xFF"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = invalid_utf8_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn all_invalid_utf8() {
        let inputs = invalid_utf8_inputs();
        for input in &inputs {
            assert!(!input.data.is_empty());
            assert!(std::str::from_utf8(&input.data).is_err());
        }
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = invalid_utf8_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
