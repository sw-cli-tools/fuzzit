use crate::CaseInput;

pub fn whitespace_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b" ".to_vec(), "single space"),
        CaseInput::new(b"\t".to_vec(), "single tab"),
        CaseInput::new(b"\n".to_vec(), "single newline"),
        CaseInput::new(b"  ".to_vec(), "two spaces"),
        CaseInput::new(b"\t\t".to_vec(), "two tabs"),
        CaseInput::new(b" \t ".to_vec(), "space-tab-space"),
        CaseInput::new(b"  \n  ".to_vec(), "spaces-newline-spaces"),
        CaseInput::new(b"\r\n".to_vec(), "carriage return newline"),
        CaseInput::new(b"    ".to_vec(), "four spaces"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = whitespace_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_space() {
        let inputs = whitespace_inputs();
        assert!(inputs.iter().any(|i| i.data == b" "));
    }

    #[test]
    fn contains_tab() {
        let inputs = whitespace_inputs();
        assert!(inputs.iter().any(|i| i.data == b"\t"));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = whitespace_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
