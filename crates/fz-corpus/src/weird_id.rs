use crate::CaseInput;

pub fn weird_identifiers() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"if".to_vec(), "keyword 'if'"),
        CaseInput::new(b"for".to_vec(), "keyword 'for'"),
        CaseInput::new(b"while".to_vec(), "keyword 'while'"),
        CaseInput::new(b"return".to_vec(), "keyword 'return'"),
        CaseInput::new(b"fn".to_vec(), "keyword 'fn'"),
        CaseInput::new(b"class".to_vec(), "keyword 'class'"),
        CaseInput::new(b"import".to_vec(), "keyword 'import'"),
        CaseInput::new(b"_".to_vec(), "single underscore"),
        CaseInput::new(b"__".to_vec(), "double underscore"),
        CaseInput::new(b"___".to_vec(), "triple underscore"),
        CaseInput::new(b"_0".to_vec(), "underscore digit"),
        CaseInput::new(b"$".to_vec(), "dollar sign"),
        CaseInput::new(b"".to_vec(), "empty identifier"),
        CaseInput::new(b"0x".to_vec(), "hex prefix only"),
        CaseInput::new(b"0b".to_vec(), "binary prefix only"),
        CaseInput::new(b"i".to_vec(), "imaginary suffix only"),
        CaseInput::new(
            "a".repeat(1000).into_bytes(),
            "very long identifier (1000 chars)",
        ),
        CaseInput::new(b"O0O0O0O0".to_vec(), "confusing O/0 mix"),
        CaseInput::new(b"l1l1l1l1".to_vec(), "confusing l/1 mix"),
        CaseInput::new(b"rn".to_vec(), "looks like m"),
        CaseInput::new(
            b"\xc3\xa9\xc3\xa1\xc3\xbc".to_vec(),
            "unicode identifier (eacute, aacute, uumlaut)",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = weird_identifiers();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_keyword() {
        let inputs = weird_identifiers();
        assert!(inputs.iter().any(|i| i.data == b"if"));
    }

    #[test]
    fn contains_underscore() {
        let inputs = weird_identifiers();
        assert!(inputs.iter().any(|i| i.data == b"_"));
    }

    #[test]
    fn contains_empty() {
        let inputs = weird_identifiers();
        assert!(inputs.iter().any(|i| i.data.is_empty()));
    }

    #[test]
    fn contains_very_long() {
        let inputs = weird_identifiers();
        assert!(inputs.iter().any(|i| i.data.len() == 1000));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = weird_identifiers();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
