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
