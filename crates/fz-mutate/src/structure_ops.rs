pub fn nest(input: &[u8], depth: usize) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input).to_string();
    let mut out = input_str;
    for _ in 0..depth {
        out = format!("({out})");
    }
    out.into_bytes()
}

pub fn numeric_substitute(input: &[u8]) -> Vec<u8> {
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return input.to_vec(),
    };
    let boundaries = [
        "0",
        "-1",
        "1",
        "9223372036854775807",
        "-9223372036854775808",
    ];
    let mut result = String::new();
    let mut num_buf = String::new();
    let mut in_number = false;

    for ch in input_str.chars() {
        if ch.is_ascii_digit() || (ch == '-' && !in_number && num_buf.is_empty()) {
            num_buf.push(ch);
            in_number = true;
        } else {
            if in_number && !num_buf.is_empty() {
                let replacement = boundaries[num_buf.len() % boundaries.len()];
                result.push_str(replacement);
                num_buf.clear();
            }
            in_number = false;
            result.push(ch);
        }
    }
    if in_number && !num_buf.is_empty() {
        let replacement = boundaries[num_buf.len() % boundaries.len()];
        result.push_str(replacement);
    }
    result.into_bytes()
}

pub fn delimiter_confuse(input: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input);
    let mut result = input_str.to_string();
    result = result.replace("(", "[").replace(")", "]");
    result = result.replace("\"", "'");
    result.into_bytes()
}

pub fn encoding_corrupt(input: &[u8]) -> Vec<u8> {
    if input.is_empty() {
        return input.to_vec();
    }
    let mut out = input.to_vec();
    let idx = input.len() / 2;
    if idx < out.len() {
        out[idx] = 0xC0;
    }
    out
}
