use super::{nh, poly1305};

mod avx2;
mod portable;

pub const NHPOLY_KEY_BYTES: usize = poly1305::POLY_KEY_BYTES + nh::NH_KEY_BYTES_PER_MESSAGE;

const NHPOLY_HASHES_PER_POLY: usize = 16; // 16 * (4 * u64 sums) polys over 512 bytes

pub struct Hasher<'t> {
    key: &'t [u8],
}

impl<'t> Hasher<'t> {
    pub fn hash(&self, out: &mut [u8; 16], msg: &[u8]) {
        #[cfg(target_arch = "x86_64")]
        {
            if is_x86_feature_detected!("avx2") {
                return unsafe { self.hash_avx2(out, msg) };
            }
        }
        self.hash_portable(out, msg)
    }

    pub fn new(key: &[u8]) -> Hasher<'_> {
        assert_eq!(key.len(), NHPOLY_KEY_BYTES);
        Hasher { key }
    }
}
