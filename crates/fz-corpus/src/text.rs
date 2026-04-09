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

pub fn escape_inputs() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"\\".to_vec(), "single backslash"),
        CaseInput::new(b"\\n".to_vec(), "backslash-n"),
        CaseInput::new(b"\\t".to_vec(), "backslash-t"),
        CaseInput::new(b"\\r".to_vec(), "backslash-r"),
        CaseInput::new(b"\\0".to_vec(), "backslash-zero"),
        CaseInput::new(b"\\\\\\".to_vec(), "escaped backslash"),
        CaseInput::new(b"\\\"".to_vec(), "escaped double quote"),
        CaseInput::new(b"\\'".to_vec(), "escaped single quote"),
        CaseInput::new(b"\\x41".to_vec(), "hex escape"),
        CaseInput::new(b"\\u0041".to_vec(), "unicode escape"),
        CaseInput::new(b"\\u{0041}".to_vec(), "unicode brace escape"),
        CaseInput::new(b"\\\\\\\\\\\\\\\\\\\\".to_vec(), "many backslashes"),
        CaseInput::new(b"'\\'".to_vec(), "single-quoted backslash"),
        CaseInput::new(b"\"\\n\\n\\n\"".to_vec(), "quoted newlines"),
    ]
}

pub fn weird_identifiers() -> Vec<CaseInput> {
    vec![
        CaseInput::new(b"if".to_vec(), "keyword 'if'"),
        CaseInput::new(b"for".to_vec(), "keyword 'for'"),
        CaseInput::new(b"while".to_vec(), "keyword 'while'"),
        CaseInput::new(b"return".to_vec(), "keyword 'return'"),
        CaseInput::new(b"fn".to_vec(), "keyword 'fn'"),
        CaseInput::new(b"class".to_vec(), "keyword 'class'"),
        CaseInput::new(b"import".to_vec(), "keyword 'import'"),
        CaseInput::new(b"_".to_vec(), "single underscore"),
        CaseInput::new(b"__".to_vec(), "double underscore"),
        CaseInput::new(b"___".to_vec(), "triple underscore"),
        CaseInput::new(b"_0".to_vec(), "underscore digit"),
        CaseInput::new(b"$".to_vec(), "dollar sign"),
        CaseInput::new(b"".to_vec(), "empty identifier"),
        CaseInput::new(b"0x".to_vec(), "hex prefix only"),
        CaseInput::new(b"0b".to_vec(), "binary prefix only"),
        CaseInput::new(b"i".to_vec(), "imaginary suffix only"),
        CaseInput::new(
            "a".repeat(1000).into_bytes(),
            "very long identifier (1000 chars)",
        ),
        CaseInput::new(b"O0O0O0O0".to_vec(), "confusing O/0 mix"),
        CaseInput::new(b"l1l1l1l1".to_vec(), "confusing l/1 mix"),
        CaseInput::new(b"rn".to_vec(), "looks like m"),
        CaseInput::new(
            b"\xc3\xa9\xc3\xa1\xc3\xbc".to_vec(),
            "unicode identifier (eacute, aacute, uumlaut)",
        ),
    ]
}
