use crate::CaseInput;

pub fn empty_inputs() -> Vec<CaseInput> {
    vec![CaseInput::new(vec![], "empty input")]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = empty_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_empty_vec() {
        let inputs = empty_inputs();
        assert!(inputs.iter().any(|i| i.data.is_empty()));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = empty_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
