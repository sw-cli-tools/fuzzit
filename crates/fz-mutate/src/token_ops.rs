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
