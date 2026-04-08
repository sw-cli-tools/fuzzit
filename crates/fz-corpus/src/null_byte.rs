use crate::CaseInput;

pub fn null_byte_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(vec![0], "single null byte"),
        CaseInput::new(vec![0, 0], "double null byte"),
        CaseInput::new(vec![b'a', 0], "null after char"),
        CaseInput::new(vec![0, b'a'], "null before char"),
        CaseInput::new(b"hello\0world".to_vec(), "null mid-string"),
        CaseInput::new(vec![0; 10], "ten null bytes"),
        CaseInput::new(vec![0; 256], "256 null bytes"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = null_byte_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_single_null() {
        let inputs = null_byte_inputs();
        assert!(inputs.iter().any(|i| i.data == vec![0]));
    }

    #[test]
    fn all_contain_null() {
        let inputs = null_byte_inputs();
        for input in &inputs {
            assert!(input.data.contains(&0));
        }
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = null_byte_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
