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
            179, 2, 194, 218, 159, 54, 51, 50, 123, 45, 114, 99, 62, 240, 238, 220, 246, 63, 101,
            64, 230, 139, 251, 33, 197, 216, 140, 69, 162, 79, 0, 169
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
            2, 165, 147, 129, 174, 97, 71, 165, 75, 4, 238, 188, 170, 234, 38, 175, 126, 34, 46,
            14, 230, 217, 110, 48, 41, 208, 119, 212, 162, 172, 93, 188
        ]
    );
}
