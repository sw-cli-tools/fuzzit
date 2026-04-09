use fz_mutate::ops;

#[test]
fn byte_flip_changes_one_byte() {
    let input = b"hello";
    let out = ops::byte_flip(input, 1);
    assert_eq!(out.len(), input.len());
    assert_ne!(out, input.to_vec());
    assert_eq!(out[0], input[0]);
    assert_eq!(out[2], input[2]);
}

#[test]
fn byte_flip_empty() {
    let out = ops::byte_flip(b"", 0);
    assert!(out.is_empty());
}

#[test]
fn byte_flip_out_of_bounds() {
    let out = ops::byte_flip(b"a", 5);
    assert_eq!(out, b"a");
}

#[test]
fn bit_flip_changes_one_bit() {
    let input = vec![0b00000000];
    let out = ops::bit_flip(&input, 0, 0);
    assert_eq!(out, vec![0b00000001]);
}

#[test]
fn bit_flip_empty() {
    let out = ops::bit_flip(b"", 0, 0);
    assert!(out.is_empty());
}

#[test]
fn delete_token_removes_one() {
    let input = b"a,b,c";
    let out = ops::delete_token(input, b',');
    assert_eq!(out.len(), 3);
    assert_eq!(out, b"a,c");
}

#[test]
fn delete_token_empty() {
    let out = ops::delete_token(b"", b',');
    assert!(out.is_empty());
}

#[test]
fn delete_token_single() {
    let out = ops::delete_token(b"hello", b',');
    assert_eq!(out, b"hello");
}

#[test]
fn duplicate_token_increases_length() {
    let input = b"a,b,c";
    let out = ops::duplicate_token(input, b',');
    assert!(out.len() > input.len());
}

#[test]
fn duplicate_token_empty() {
    let out = ops::duplicate_token(b"", b',');
    assert_eq!(out, b"");
}

#[test]
fn splice_inserts_bytes() {
    let input = b"abcdef";
    let other = b"XYZ";
    let out = ops::splice(input, other, 0, 3);
    assert!(out.len() > input.len());
    assert!(out.windows(3).any(|w| w == b"XYZ"));
}

#[test]
fn splice_empty_input() {
    let out = ops::splice(b"", b"XYZ", 0, 3);
    assert!(out.is_empty());
}

#[test]
fn nest_wraps() {
    let input = b"hello";
    let out = ops::nest(input, 1);
    assert_eq!(out, b"(hello)");
}

#[test]
fn nest_multiple() {
    let input = b"hello";
    let out = ops::nest(input, 2);
    assert_eq!(out, b"((hello))");
}

#[test]
fn nest_zero() {
    let input = b"hello";
    let out = ops::nest(input, 0);
    assert_eq!(out, b"hello");
}

#[test]
fn numeric_substitute_replaces() {
    let input = b"foo 42 bar";
    let out = ops::numeric_substitute(input);
    assert_ne!(out, b"foo 42 bar".to_vec());
    assert!(out.starts_with(b"foo "));
}

#[test]
fn numeric_substitute_empty() {
    let out = ops::numeric_substitute(b"");
    assert!(out.is_empty());
}

#[test]
fn numeric_substitute_no_numbers() {
    let input = b"hello world";
    let out = ops::numeric_substitute(input);
    assert_eq!(out, b"hello world".to_vec());
}

#[test]
fn delimiter_confuse_swaps() {
    let input = b"(hello)";
    let out = ops::delimiter_confuse(input);
    assert_eq!(out, b"[hello]");
}

#[test]
fn delimiter_confuse_quotes() {
    let input = b"\"hello\"";
    let out = ops::delimiter_confuse(input);
    assert_eq!(out, b"'hello'");
}

#[test]
fn delimiter_confuse_empty() {
    let out = ops::delimiter_confuse(b"");
    assert!(out.is_empty());
}

#[test]
fn encoding_corrupt_changes_mid() {
    let input = b"abcdefgh";
    let out = ops::encoding_corrupt(input);
    assert_eq!(out.len(), input.len());
    assert_eq!(out[0], input[0]);
    assert_eq!(out[4], 0xC0);
}

#[test]
fn encoding_corrupt_empty() {
    let out = ops::encoding_corrupt(b"");
    assert!(out.is_empty());
}
