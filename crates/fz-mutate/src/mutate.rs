use rand::RngCore;

use crate::ops;

const NUM_OPERATORS: usize = 9;

pub fn mutate(input: &[u8], rng: &mut impl RngCore) -> Vec<u8> {
    if input.is_empty() {
        return input.to_vec();
    }
    let op = rng.next_u32() as usize % NUM_OPERATORS;
    match op {
        0 => {
            let idx = rng.next_u32() as usize % input.len();
            ops::byte_flip(input, idx)
        }
        1 => {
            let idx = rng.next_u32() as usize % input.len();
            let bit = rng.next_u32() as u8 % 8;
            ops::bit_flip(input, idx, bit)
        }
        2 => ops::delete_token(input, b' '),
        3 => ops::duplicate_token(input, b' '),
        4 => {
            let other = input;
            let pos = rng.next_u32() as usize % (other.len().saturating_add(1));
            let len = rng.next_u32() as usize % (other.len().saturating_add(1));
            ops::splice(input, other, pos, len)
        }
        5 => {
            let depth = (rng.next_u32() as usize % 3) + 1;
            ops::nest(input, depth)
        }
        6 => ops::numeric_substitute(input),
        7 => ops::delimiter_confuse(input),
        8 => ops::encoding_corrupt(input),
        _ => input.to_vec(),
    }
}

pub fn mutate_n(input: &[u8], count: usize, rng: &mut impl RngCore) -> Vec<Vec<u8>> {
    (0..count).map(|_| mutate(input, rng)).collect()
}
