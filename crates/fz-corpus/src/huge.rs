use crate::CaseInput;

pub fn huge_input(size: usize) -> CaseInput {
    CaseInput::new(vec![b'A'; size], format!("huge input ({size} bytes)"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_size() {
        let input = huge_input(1024);
        assert_eq!(input.data.len(), 1024);
    }

    #[test]
    fn all_same_byte() {
        let input = huge_input(100);
        assert!(input.data.iter().all(|&b| b == b'A'));
    }

    #[test]
    fn description_contains_size() {
        let input = huge_input(500);
        assert!(input.description.contains("500"));
    }

    #[test]
    fn handles_zero_size() {
        let input = huge_input(0);
        assert!(input.data.is_empty());
    }
}
