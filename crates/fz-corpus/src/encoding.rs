use crate::CaseInput;

pub fn invalid_utf8_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(vec![0x80], "single continuation byte"),
        CaseInput::new(vec![0xC0, 0x80], "overlong 2-byte encoding"),
        CaseInput::new(vec![0xE0, 0x80, 0x80], "overlong 3-byte encoding"),
        CaseInput::new(vec![0xF0, 0x80, 0x80, 0x80], "overlong 4-byte encoding"),
        CaseInput::new(vec![0xC2], "truncated 2-byte sequence"),
        CaseInput::new(vec![0xE0, 0xA0], "truncated 3-byte sequence"),
        CaseInput::new(vec![0xF4, 0x90, 0x80, 0x80], "codepoint above U+10FFFF"),
        CaseInput::new(vec![0xED, 0xA0, 0x80], "UTF-16 surrogate (U+D800)"),
        CaseInput::new(
            vec![0xF8, 0x80, 0x80, 0x80, 0x80],
            "invalid 5-byte sequence",
        ),
        CaseInput::new(vec![0xFF, 0xFE], "BOM in wrong order"),
        CaseInput::new(vec![0xFE, 0xFF], "UTF-16 BE BOM"),
        CaseInput::new(vec![0xFF], "invalid single byte 0xFF"),
    ]
}

pub fn newline_variants() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"\n".to_vec(), "LF only"),
        CaseInput::new(b"\r".to_vec(), "CR only"),
        CaseInput::new(b"\r\n".to_vec(), "CRLF"),
        CaseInput::new(b"\n\r".to_vec(), "LF then CR"),
        CaseInput::new(b"\n\n".to_vec(), "double LF"),
        CaseInput::new(b"\r\r".to_vec(), "double CR"),
        CaseInput::new(b"\r\n\r\n".to_vec(), "double CRLF"),
        CaseInput::new(b"a\nb\nc".to_vec(), "multi-line LF"),
        CaseInput::new(b"a\r\nb\r\nc".to_vec(), "multi-line CRLF"),
        CaseInput::new(b"a\nb\r\nc".to_vec(), "mixed line endings"),
        CaseInput::new(b"\r\r\n\n\r\n".to_vec(), "chaotic line endings"),
    ]
}

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
