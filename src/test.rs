use crate::sthash::*;

#[test]
fn basic() {
    let mut seed = [0; SEED_BYTES];
    for i in 0..SEED_BYTES {
        seed[i] = i as u8;
    }
    let key = Key::from_seed(&seed, Some(b"test suite"));
    let hasher = Hasher::new(key, None);
    let h1 = hasher.hash(b"test data 1");
    let h2 = hasher.hash(b"test data 2");
    assert_ne!(h1, h2);
    assert_eq!(
        h1,
        [
            207, 49, 8, 127, 113, 64, 236, 115, 32, 134, 137, 211, 231, 179, 55, 152, 157, 237,
            108, 170, 124, 221, 19, 27, 204, 147, 234, 183, 207, 229, 205, 115
        ]
    );
}

#[test]
fn large() {
    let mut seed = [0; SEED_BYTES];
    for i in 0..SEED_BYTES {
        seed[i] = i as u8;
    }
    let key = Key::from_seed(&seed, Some(b"test suite"));
    let hasher = Hasher::new(key, None);
    let large = vec![0x42; 10_0000];
    let h1 = hasher.hash(&large);
    assert_eq!(
        h1,
        [
            110, 162, 21, 125, 173, 183, 249, 134, 212, 41, 152, 188, 190, 128, 190, 146, 78, 80,
            111, 186, 86, 150, 73, 137, 12, 42, 117, 217, 69, 154, 74, 231
        ]
    );
}
