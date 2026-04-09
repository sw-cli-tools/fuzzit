use fz_mutate::{mutate, mutate_n};

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
