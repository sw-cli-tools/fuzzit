use crate::CaseInput;

pub fn empty_inputs() -> Vec<CaseInput> {
    vec![CaseInput::new(vec![], "empty input")]
}

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

pub fn single_char_inputs() -> Vec<CaseInput> {
    (32u8..=126u8)
        .map(|b| {
            let ch = b as char;
            CaseInput::new(vec![b], format!("single char '{ch}' (0x{b:02x})"))
        })
        .collect()
}
