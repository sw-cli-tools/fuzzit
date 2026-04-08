use crate::CaseInput;

pub fn deep_nesting(depth: usize) -> Vec<CaseInput> {
    let mut inputs = Vec::new();

    let parens: Vec<u8> = "(".repeat(depth).into_bytes();
    let parens_close: Vec<u8> = ")".repeat(depth).into_bytes();
    let mut nested_parens = parens.clone();
    nested_parens.extend_from_slice(&parens_close);
    inputs.push(CaseInput::new(
        nested_parens,
        format!("nested parens depth {depth}"),
    ));

    let brackets: Vec<u8> = "[".repeat(depth).into_bytes();
    let brackets_close: Vec<u8> = "]".repeat(depth).into_bytes();
    let mut nested_brackets = brackets.clone();
    nested_brackets.extend_from_slice(&brackets_close);
    inputs.push(CaseInput::new(
        nested_brackets,
        format!("nested brackets depth {depth}"),
    ));

    let braces: Vec<u8> = "{".repeat(depth).into_bytes();
    let braces_close: Vec<u8> = "}".repeat(depth).into_bytes();
    let mut nested_braces = braces.clone();
    nested_braces.extend_from_slice(&braces_close);
    inputs.push(CaseInput::new(
        nested_braces,
        format!("nested braces depth {depth}"),
    ));

    let mut nested_mixed = Vec::new();
    for _ in 0..depth {
        nested_mixed.push(b'(');
        nested_mixed.push(b'[');
        nested_mixed.push(b'{');
    }
    for _ in 0..depth {
        nested_mixed.push(b'}');
        nested_mixed.push(b']');
        nested_mixed.push(b')');
    }
    inputs.push(CaseInput::new(
        nested_mixed,
        format!("nested mixed depth {depth}"),
    ));

    let mut unbalanced_open: Vec<u8> = "(".repeat(depth).into_bytes();
    unbalanced_open.push(b'x');
    inputs.push(CaseInput::new(
        unbalanced_open,
        format!("unbalanced open depth {depth}"),
    ));

    let mut unbalanced_close: Vec<u8> = "x".into();
    unbalanced_close.extend_from_slice(&")".repeat(depth).into_bytes());
    inputs.push(CaseInput::new(
        unbalanced_close,
        format!("unbalanced close depth {depth}"),
    ));

    inputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty() {
        let inputs = deep_nesting(3);
        assert!(!inputs.is_empty());
    }

    #[test]
    fn contains_nested_parens() {
        let inputs = deep_nesting(2);
        assert!(inputs.iter().any(|i| i.data == b"(())"));
    }

    #[test]
    fn depth_affects_size() {
        let shallow = deep_nesting(1);
        let deep = deep_nesting(5);
        let shallow_max: usize = shallow.iter().map(|i| i.data.len()).max().unwrap();
        let deep_max: usize = deep.iter().map(|i| i.data.len()).max().unwrap();
        assert!(deep_max > shallow_max);
    }

    #[test]
    fn handles_zero_depth() {
        let inputs = deep_nesting(0);
        assert!(!inputs.is_empty());
    }

    #[test]
    fn descriptions_non_empty() {
        let inputs = deep_nesting(2);
        for input in &inputs {
            assert!(!input.description.is_empty());
        }
    }
}
