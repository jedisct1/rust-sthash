use crate::sthash::*;
use rand::{thread_rng, RngCore};

#[test]
fn basic() {
    let mut seed = [0; SEED_BYTES];
    thread_rng().fill_bytes(&mut seed);
    let key = Key::from_seed(&seed, Some(b"test suite"));
    let hasher = Hasher::new(key, None);
    let h1 = hasher.hash(b"test data 1");
    let h2 = hasher.hash(b"test data 2");
    assert_ne!(h1, h2);
}

#[test]
fn large() {
    let mut seed = [0; SEED_BYTES];
    thread_rng().fill_bytes(&mut seed);
    let key = Key::from_seed(&seed, Some(b"test suite"));
    let hasher = Hasher::new(key, None);
    let large = vec![0x42; 10_0000];
    let h1 = hasher.hash(&large);
    let h2 = hasher.hash(b"test data 2");
    assert_ne!(h1, h2);
}
