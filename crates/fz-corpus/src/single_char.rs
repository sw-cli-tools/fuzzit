use crate::CaseInput;

pub fn single_char_inputs() -> Vec<CaseInput> {
    (32u8..=126u8)
        .map(|b| {
            let ch = b as char;
            CaseInput::new(vec![b], format!("single char '{ch}' (0x{b:02x})"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = single_char_inputs();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn covers_printable_ascii() {
        let inputs = single_char_inputs();
        assert_eq!(inputs.len(), 95);
    }

    #[test]
    fn each_is_single_byte() {
        let inputs = single_char_inputs();
        for input in &inputs {
            assert_eq!(input.data.len(), 1);
        }
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = single_char_inputs();
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
