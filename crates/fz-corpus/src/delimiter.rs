use crate::CaseInput;

pub fn delimiter_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"()".to_vec(), "empty parens"),
        CaseInput::new(b"{}".to_vec(), "empty braces"),
        CaseInput::new(b"[]".to_vec(), "empty brackets"),
        CaseInput::new(b"\"\"".to_vec(), "empty double quotes"),
        CaseInput::new(b"''".to_vec(), "empty single quotes"),
        CaseInput::new(b"()()".to_vec(), "double parens"),
        CaseInput::new(b"{{}}".to_vec(), "nested braces"),
        CaseInput::new(b"[][]".to_vec(), "double brackets"),
        CaseInput::new(b"(())".to_vec(), "nested parens"),
        CaseInput::new(b"[[[]]]".to_vec(), "deep nested brackets"),
        CaseInput::new(b"(())[()]".to_vec(), "multiple nested parens"),
        CaseInput::new(b"\"'\"'\"".to_vec(), "alternating quotes"),
        CaseInput::new(b"{{{}}}".to_vec(), "unbalanced braces (extra open)"),
        CaseInput::new(b"}}}".to_vec(), "unbalanced braces (extra close)"),
        CaseInput::new(b"(((((((((".to_vec(), "many open parens"),
        CaseInput::new(b"))))))))".to_vec(), "many close parens"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = delimiter_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_parens() {
        let inputs = delimiter_inputs();
        assert!(inputs.iter().any(|i| i.data == b"()"));
    }

    #[test]
    fn contains_braces() {
        let inputs = delimiter_inputs();
        assert!(inputs.iter().any(|i| i.data == b"{}"));
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = delimiter_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
