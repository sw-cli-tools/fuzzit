use crate::CaseInput;

pub fn escape_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"\\".to_vec(), "single backslash"),
        CaseInput::new(b"\\n".to_vec(), "backslash-n"),
        CaseInput::new(b"\\t".to_vec(), "backslash-t"),
        CaseInput::new(b"\\r".to_vec(), "backslash-r"),
        CaseInput::new(b"\\0".to_vec(), "backslash-zero"),
        CaseInput::new(b"\\\\".to_vec(), "escaped backslash"),
        CaseInput::new(b"\\\"".to_vec(), "escaped double quote"),
        CaseInput::new(b"\\'".to_vec(), "escaped single quote"),
        CaseInput::new(b"\\x41".to_vec(), "hex escape"),
        CaseInput::new(b"\\u0041".to_vec(), "unicode escape"),
        CaseInput::new(b"\\u{0041}".to_vec(), "unicode brace escape"),
        CaseInput::new(b"\\\\\\\\\\\\\\\\\\\\".to_vec(), "many backslashes"),
        CaseInput::new(b"'\\'".to_vec(), "single-quoted backslash"),
        CaseInput::new(b"\"\\n\\n\\n\"".to_vec(), "quoted newlines"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = escape_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_single_backslash() {
        let inputs = escape_inputs();
        assert!(inputs.iter().any(|i| i.data == b"\\"));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = escape_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
