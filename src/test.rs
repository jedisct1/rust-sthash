use crate::sthash::*;

#[test]
fn basic() {
    let seed = [0x42; SEED_BYTES];
    let key = Key::from_seed(&seed, Some(b"test suite"));
    let hasher = Hasher::new(key, None);
    let h1 = hasher.hash(b"test data 1");
    let h2 = hasher.hash(b"test data 2");
    assert_ne!(h1, h2);
}
