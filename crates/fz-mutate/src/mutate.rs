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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutate_returns_same_length_or_larger() {
        let input = b"hello world foo bar baz";
        let mut rng = rand::rng();
        for _ in 0..100 {
            let out = mutate(input, &mut rng);
            assert!(out.len() >= 5);
        }
    }

    #[test]
    fn mutate_empty_returns_empty() {
        let mut rng = rand::rng();
        let out = mutate(b"", &mut rng);
        assert!(out.is_empty());
    }

    #[test]
    fn mutate_n_returns_count() {
        let input = b"test input data";
        let mut rng = rand::rng();
        let results = mutate_n(input, 50, &mut rng);
        assert_eq!(results.len(), 50);
    }

    #[test]
    fn mutate_n_zero_returns_empty() {
        let mut rng = rand::rng();
        let results = mutate_n(b"test", 0, &mut rng);
        assert!(results.is_empty());
    }

    #[test]
    fn mutate_single_byte() {
        let input = b"X";
        let mut rng = rand::rng();
        for _ in 0..20 {
            let out = mutate(input, &mut rng);
            assert!(!out.is_empty());
        }
    }
}
