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

fn nested_pair(open: u8, close: u8, depth: usize, label: &str) -> CaseInput {
    let mut data = vec![open; depth];
    data.extend_from_slice(&vec![close; depth]);
    CaseInput::new(data, format!("nested {label} depth {depth}"))
}

fn nested_mixed(depth: usize) -> CaseInput {
    let mut data = Vec::new();
    for _ in 0..depth {
        data.extend_from_slice(b"([{");
    }
    for _ in 0..depth {
        data.extend_from_slice(b")]}");
    }
    CaseInput::new(data, format!("nested mixed depth {depth}"))
}

pub fn deep_nesting(depth: usize) -> Vec<CaseInput> {
    let mut inputs = vec![
        nested_pair(b'(', b')', depth, "parens"),
        nested_pair(b'[', b']', depth, "brackets"),
        nested_pair(b'{', b'}', depth, "braces"),
        nested_mixed(depth),
    ];

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

pub fn huge_input(size: usize) -> CaseInput {
    CaseInput::new(vec![b'A'; size], format!("huge input ({size} bytes)"))
}
