pub fn byte_flip(input: &[u8], index: usize) -> Vec<u8> {
    let mut out = input.to_vec();
    if index < out.len() {
        out[index] = out[index].wrapping_add(1);
    }
    out
}

pub fn bit_flip(input: &[u8], index: usize, bit: u8) -> Vec<u8> {
    let mut out = input.to_vec();
    if index < out.len() && bit < 8 {
        out[index] ^= 1 << bit;
    }
    out
}

pub fn delete_token(input: &[u8], sep: u8) -> Vec<u8> {
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return input.to_vec(),
    };
    let tokens: Vec<&str> = input_str.split(sep as char).collect();
    if tokens.len() <= 1 {
        return input.to_vec();
    }
    let remove_idx = tokens.len() / 2;
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i == remove_idx {
            continue;
        }
        if i > 0 {
            result.push(sep as char);
        }
        result.push_str(token);
    }
    result.into_bytes()
}

pub fn duplicate_token(input: &[u8], sep: u8) -> Vec<u8> {
    let input_str = match std::str::from_utf8(input) {
        Ok(s) => s,
        Err(_) => return input.to_vec(),
    };
    if input_str.is_empty() {
        return input.to_vec();
    }
    let tokens: Vec<&str> = input_str.split(sep as char).collect();
    let dup_idx = tokens.len() / 2;
    let mut result = String::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            result.push(sep as char);
        }
        result.push_str(token);
        if i == dup_idx {
            result.push(sep as char);
            result.push_str(token);
        }
    }
    result.into_bytes()
}

pub fn splice(input: &[u8], other: &[u8], pos: usize, len: usize) -> Vec<u8> {
    let splice_end = (pos + len).min(other.len());
    if pos >= other.len() || input.is_empty() {
        return input.to_vec();
    }
    let insert_at = pos % input.len().saturating_add(1);
    let mut out = Vec::with_capacity(input.len() + splice_end - pos);
    out.extend_from_slice(&input[..insert_at]);
    out.extend_from_slice(&other[pos..splice_end]);
    out.extend_from_slice(&input[insert_at..]);
    out
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_flip_changes_one_byte() {
        let input = b"hello";
        let out = byte_flip(input, 1);
        assert_eq!(out.len(), input.len());
        assert_ne!(out, input.to_vec());
        assert_eq!(out[0], input[0]);
        assert_eq!(out[2], input[2]);
    }

    #[test]
    fn byte_flip_empty() {
        let out = byte_flip(b"", 0);
        assert!(out.is_empty());
    }

    #[test]
    fn byte_flip_out_of_bounds() {
        let out = byte_flip(b"a", 5);
        assert_eq!(out, b"a");
    }

    #[test]
    fn bit_flip_changes_one_bit() {
        let input = vec![0b00000000];
        let out = bit_flip(&input, 0, 0);
        assert_eq!(out, vec![0b00000001]);
    }

    #[test]
    fn bit_flip_empty() {
        let out = bit_flip(b"", 0, 0);
        assert!(out.is_empty());
    }

    #[test]
    fn delete_token_removes_one() {
        let input = b"a,b,c";
        let out = delete_token(input, b',');
        assert_eq!(out.len(), 3);
        assert_eq!(out, b"a,c");
    }

    #[test]
    fn delete_token_empty() {
        let out = delete_token(b"", b',');
        assert!(out.is_empty());
    }

    #[test]
    fn delete_token_single() {
        let out = delete_token(b"hello", b',');
        assert_eq!(out, b"hello");
    }

    #[test]
    fn duplicate_token_increases_length() {
        let input = b"a,b,c";
        let out = duplicate_token(input, b',');
        assert!(out.len() > input.len());
    }

    #[test]
    fn duplicate_token_empty() {
        let out = duplicate_token(b"", b',');
        assert_eq!(out, b"");
    }

    #[test]
    fn splice_inserts_bytes() {
        let input = b"abcdef";
        let other = b"XYZ";
        let out = splice(input, other, 0, 3);
        assert!(out.len() > input.len());
        assert!(out.windows(3).any(|w| w == b"XYZ"));
    }

    #[test]
    fn splice_empty_input() {
        let out = splice(b"", b"XYZ", 0, 3);
        assert!(out.is_empty());
    }

    #[test]
    fn nest_wraps() {
        let input = b"hello";
        let out = nest(input, 1);
        assert_eq!(out, b"(hello)");
    }

    #[test]
    fn nest_multiple() {
        let input = b"hello";
        let out = nest(input, 2);
        assert_eq!(out, b"((hello))");
    }

    #[test]
    fn nest_zero() {
        let input = b"hello";
        let out = nest(input, 0);
        assert_eq!(out, b"hello");
    }

    #[test]
    fn numeric_substitute_replaces() {
        let input = b"foo 42 bar";
        let out = numeric_substitute(input);
        assert_ne!(out, b"foo 42 bar".to_vec());
        assert!(out.starts_with(b"foo "));
    }

    #[test]
    fn numeric_substitute_empty() {
        let out = numeric_substitute(b"");
        assert!(out.is_empty());
    }

    #[test]
    fn numeric_substitute_no_numbers() {
        let input = b"hello world";
        let out = numeric_substitute(input);
        assert_eq!(out, b"hello world".to_vec());
    }

    #[test]
    fn delimiter_confuse_swaps() {
        let input = b"(hello)";
        let out = delimiter_confuse(input);
        assert_eq!(out, b"[hello]");
    }

    #[test]
    fn delimiter_confuse_quotes() {
        let input = b"\"hello\"";
        let out = delimiter_confuse(input);
        assert_eq!(out, b"'hello'");
    }

    #[test]
    fn delimiter_confuse_empty() {
        let out = delimiter_confuse(b"");
        assert!(out.is_empty());
    }

    #[test]
    fn encoding_corrupt_changes_mid() {
        let input = b"abcdefgh";
        let out = encoding_corrupt(input);
        assert_eq!(out.len(), input.len());
        assert_eq!(out[0], input[0]);
        assert_eq!(out[4], 0xC0);
    }

    #[test]
    fn encoding_corrupt_empty() {
        let out = encoding_corrupt(b"");
        assert!(out.is_empty());
    }
}
