use crate::CaseInput;

pub fn numeric_boundary_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"0".to_vec(), "zero"),
        CaseInput::new(b"-1".to_vec(), "negative one"),
        CaseInput::new(b"1".to_vec(), "positive one"),
        CaseInput::new(i64::MAX.to_string().into_bytes(), "i64 max"),
        CaseInput::new(i64::MIN.to_string().into_bytes(), "i64 min"),
        CaseInput::new(u64::MAX.to_string().into_bytes(), "u64 max"),
        CaseInput::new(b"0.0".to_vec(), "float zero"),
        CaseInput::new(b"-0.0".to_vec(), "negative float zero"),
        CaseInput::new(b"inf".to_vec(), "infinity string"),
        CaseInput::new(b"-inf".to_vec(), "negative infinity string"),
        CaseInput::new(b"NaN".to_vec(), "NaN string"),
        CaseInput::new(b"9999999999999999999".to_vec(), "very large number"),
        CaseInput::new(b"-9999999999999999999".to_vec(), "very large negative"),
        CaseInput::new(b"0.0000000001".to_vec(), "very small float string"),
        CaseInput::new(b"1e308".to_vec(), "near f64 max exponent"),
        CaseInput::new(b"-1e308".to_vec(), "near f64 min exponent"),
        CaseInput::new(b"18446744073709551616".to_vec(), "u64 max + 1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = numeric_boundary_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_zero() {
        let inputs = numeric_boundary_inputs();
        assert!(inputs.iter().any(|i| i.data == b"0"));
    }

    #[test]
    fn contains_negative_one() {
        let inputs = numeric_boundary_inputs();
        assert!(inputs.iter().any(|i| i.data == b"-1"));
    }

    #[test]
    fn contains_i64_max() {
        let inputs = numeric_boundary_inputs();
        assert!(
            inputs
                .iter()
                .any(|i| { std::str::from_utf8(&i.data).unwrap() == i64::MAX.to_string() })
        );
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = numeric_boundary_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
