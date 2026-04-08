use crate::CaseInput;

pub fn newline_variants() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"\n".to_vec(), "LF only"),
        CaseInput::new(b"\r".to_vec(), "CR only"),
        CaseInput::new(b"\r\n".to_vec(), "CRLF"),
        CaseInput::new(b"\n\r".to_vec(), "LF then CR"),
        CaseInput::new(b"\n\n".to_vec(), "double LF"),
        CaseInput::new(b"\r\r".to_vec(), "double CR"),
        CaseInput::new(b"\r\n\r\n".to_vec(), "double CRLF"),
        CaseInput::new(b"a\nb\nc".to_vec(), "multi-line LF"),
        CaseInput::new(b"a\r\nb\r\nc".to_vec(), "multi-line CRLF"),
        CaseInput::new(b"a\nb\r\nc".to_vec(), "mixed line endings"),
        CaseInput::new(b"\r\r\n\n\r\n".to_vec(), "chaotic line endings"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = newline_variants();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_lf() {
        let inputs = newline_variants();
        assert!(inputs.iter().any(|i| i.data == b"\n"));
    }

    #[test]
    fn contains_crlf() {
        let inputs = newline_variants();
        assert!(inputs.iter().any(|i| i.data == b"\r\n"));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = newline_variants();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
