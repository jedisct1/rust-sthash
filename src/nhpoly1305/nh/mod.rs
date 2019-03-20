#[cfg(target_arch = "x86_64")]
mod avx2;
mod portable;

use byteorder::{ByteOrder, LittleEndian};

pub const NH_NUM_PASSES: usize = 4;
pub const NH_PAIR_STRIDE: usize = 2;
pub const NH_MESSAGE_UNIT: usize = NH_PAIR_STRIDE * 8; // 16
pub const NH_NUM_STRIDES: usize = 64;
pub const NH_MESSAGE_DWORDS: usize = NH_PAIR_STRIDE * 2 * NH_NUM_STRIDES;
pub const NH_MESSAGE_BYTES: usize = NH_MESSAGE_DWORDS * 4; // 1Kb
pub const NH_KEYS_PER_MESSAGE: usize = NH_MESSAGE_BYTES;
pub const NH_KEY_BYTES_PER_MESSAGE: usize = 4 * NH_KEYS_PER_MESSAGE;
pub const NH_OUTPUT_BYTES: usize = 8 * 4;

pub struct Hasher {
    key: [u32; NH_KEYS_PER_MESSAGE],
}

impl Hasher {
    pub fn new(key: &[u8]) -> Hasher {
        debug_assert!(key.len() == NH_KEY_BYTES_PER_MESSAGE);
        let mut key_u32 = [0u32; NH_KEYS_PER_MESSAGE];
        for i in 0..NH_KEYS_PER_MESSAGE {
            key_u32[i] = LittleEndian::read_u32(&key[i * 4..]);
        }
        Hasher { key: key_u32 }
    }
}

#[test]
fn basic_small() {
    let key = vec![1; NH_KEY_BYTES_PER_MESSAGE];
    let h = Hasher::new(&key);
    let msg = vec![42; 64];
    let mut out = Vec::new();
    h.hash(&mut out, &msg);
    assert_eq!(
        out,
        [
            200, 201, 203, 205, 63, 62, 60, 58, 200, 201, 203, 205, 63, 62, 60, 58, 200, 201, 203,
            205, 63, 62, 60, 58, 200, 201, 203, 205, 63, 62, 60, 58
        ]
    );
}

#[cfg(target_arch = "x86_64")]
#[test]
fn test_avx2() {
    use rand::{thread_rng, RngCore};

    let mut key = vec![1; NH_KEY_BYTES_PER_MESSAGE];
    thread_rng().fill_bytes(&mut key);
    let h = Hasher::new(&key);
    let mut msg = vec![0; 256];
    thread_rng().fill_bytes(&mut msg);
    let mut out = Vec::new();
    h.hash(&mut out, &msg);
    let mut out_avx2 = Vec::new();
    unsafe { h.hash_avx2(&mut out_avx2, &msg) };
    assert_eq!(out, out_avx2);
}
