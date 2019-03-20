use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub const NH_NUM_PASSES: usize = 4;
pub const NH_PAIR_STRIDE: usize = 2;
pub const NH_MESSAGE_UNIT: usize = NH_PAIR_STRIDE * 8; // 16
pub const NH_NUM_STRIDES: usize = 64;
pub const NH_MESSAGE_DWORDS: usize = NH_PAIR_STRIDE * 2 * NH_NUM_STRIDES;
pub const NH_MESSAGE_BYTES: usize = NH_MESSAGE_DWORDS * 4; // 1Kb
pub const NH_KEYS_PER_MESSAGE: usize = NH_MESSAGE_BYTES;
pub const NH_KEY_BYTES_PER_MESSAGE: usize = 4 * NH_KEYS_PER_MESSAGE;
pub const NH_OUTPUT_BYTES: usize = 8 * 4;

pub struct Hash {
    key: [u32; NH_KEYS_PER_MESSAGE],
}

impl Hash {
    #[inline(always)]
    pub fn hash(&self, out: &mut Vec<u8>, msg: &[u8]) {
        let mut cursor = Cursor::new(msg);
        let mut remaining = msg.len();
        let (mut s0, mut s1, mut s2, mut s3) = (0u64, 0u64, 0u64, 0u64);
        let mut key_ = &self.key[..];

        debug_assert_eq!(NH_NUM_PASSES, 4);
        debug_assert_eq!(remaining % NH_MESSAGE_UNIT, 0);
        debug_assert!(key_.len() >= remaining / NH_MESSAGE_UNIT * 16);
        while remaining > 0 {
            let m0 = cursor.read_u32::<LittleEndian>().unwrap();
            let m1 = cursor.read_u32::<LittleEndian>().unwrap();
            let m2 = cursor.read_u32::<LittleEndian>().unwrap();
            let m3 = cursor.read_u32::<LittleEndian>().unwrap();

            s0 = s0.wrapping_add(
                ((m0.wrapping_add(key_[0])) as u64).wrapping_mul(m2.wrapping_add(key_[2]) as u64),
            );
            s1 = s1.wrapping_add(
                ((m0.wrapping_add(key_[4])) as u64).wrapping_mul(m2.wrapping_add(key_[6]) as u64),
            );
            s2 = s2.wrapping_add(
                ((m0.wrapping_add(key_[8])) as u64).wrapping_mul(m2.wrapping_add(key_[10]) as u64),
            );
            s3 = s3.wrapping_add(
                ((m0.wrapping_add(key_[12])) as u64).wrapping_mul(m2.wrapping_add(key_[14]) as u64),
            );
            s0 = s0.wrapping_add(
                ((m1.wrapping_add(key_[1])) as u64).wrapping_mul(m3.wrapping_add(key_[3]) as u64),
            );
            s1 = s1.wrapping_add(
                ((m1.wrapping_add(key_[5])) as u64).wrapping_mul(m3.wrapping_add(key_[7]) as u64),
            );
            s2 = s2.wrapping_add(
                ((m1.wrapping_add(key_[9])) as u64).wrapping_mul(m3.wrapping_add(key_[11]) as u64),
            );
            s3 = s3.wrapping_add(
                ((m1.wrapping_add(key_[13])) as u64).wrapping_mul(m3.wrapping_add(key_[15]) as u64),
            );

            key_ = &key_[NH_MESSAGE_UNIT / 4..];
            remaining -= NH_MESSAGE_UNIT;
        }
        out.write_u64::<LittleEndian>(s0).unwrap();
        out.write_u64::<LittleEndian>(s1).unwrap();
        out.write_u64::<LittleEndian>(s2).unwrap();
        out.write_u64::<LittleEndian>(s3).unwrap();
    }

    #[allow(dead_code)]
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "avx2")]
    #[inline]
    pub unsafe fn hash_avx2(&self, out: &mut Vec<u8>, msg: &[u8]) {
        let mut key_ = &self.key[..];
        let mut k0 = _mm256_loadu_si256(key_.as_ptr().add(0) as *const __m256i);
        let mut k1 = _mm256_loadu_si256(key_.as_ptr().add(4) as *const __m256i);
        let mut k2;
        let mut k3;
        let mut t0;
        let mut t1;
        let mut t2;
        let mut t3;
        let mut t4;
        let mut t5;
        let mut t6;
        let mut t7;
        key_ = &key_[8..];
        let (mut sums0, mut sums1, mut sums2, mut sums3) = (
            _mm256_setzero_si256(),
            _mm256_setzero_si256(),
            _mm256_setzero_si256(),
            _mm256_setzero_si256(),
        );
        let mut msg_ = msg;
        let mut remaining = msg_.len();
        while remaining >= 64 {
            t3 = _mm256_loadu_si256(msg_.as_ptr().add(0) as *const __m256i);
            k2 = _mm256_loadu_si256(key_.as_ptr().add(0) as *const __m256i);
            k3 = _mm256_loadu_si256(key_.as_ptr().add(4) as *const __m256i);
            {
                t0 = _mm256_add_epi32(k0, t3);
                t1 = _mm256_add_epi32(k1, t3);
                t2 = _mm256_add_epi32(k2, t3);
                t3 = _mm256_add_epi32(k3, t3);
                t4 = _mm256_shuffle_epi32(t0, 0x10);
                t0 = _mm256_shuffle_epi32(t0, 0x32);
                t5 = _mm256_shuffle_epi32(t1, 0x10);
                t1 = _mm256_shuffle_epi32(t1, 0x32);
                t6 = _mm256_shuffle_epi32(t2, 0x10);
                t2 = _mm256_shuffle_epi32(t2, 0x32);
                t7 = _mm256_shuffle_epi32(t3, 0x10);
                t3 = _mm256_shuffle_epi32(t3, 0x32);
                t0 = _mm256_mul_epu32(t0, t4);
                t1 = _mm256_mul_epu32(t1, t5);
                t2 = _mm256_mul_epu32(t2, t6);
                t3 = _mm256_mul_epu32(t3, t7);
                sums0 = _mm256_add_epi64(sums0, t0);
                sums1 = _mm256_add_epi64(sums1, t1);
                sums2 = _mm256_add_epi64(sums2, t2);
                sums3 = _mm256_add_epi64(sums3, t3);
            }
            t3 = _mm256_loadu_si256(msg_.as_ptr().add(8) as *const __m256i);
            k0 = _mm256_loadu_si256(key_.as_ptr().add(8) as *const __m256i);
            k1 = _mm256_loadu_si256(key_.as_ptr().add(12) as *const __m256i);
            {
                t0 = _mm256_add_epi32(k2, t3);
                t1 = _mm256_add_epi32(k3, t3);
                t2 = _mm256_add_epi32(k0, t3);
                t3 = _mm256_add_epi32(k1, t3);
                t4 = _mm256_shuffle_epi32(t0, 0x10);
                t0 = _mm256_shuffle_epi32(t0, 0x32);
                t5 = _mm256_shuffle_epi32(t1, 0x10);
                t1 = _mm256_shuffle_epi32(t1, 0x32);
                t6 = _mm256_shuffle_epi32(t2, 0x10);
                t2 = _mm256_shuffle_epi32(t2, 0x32);
                t7 = _mm256_shuffle_epi32(t3, 0x10);
                t3 = _mm256_shuffle_epi32(t3, 0x32);
                t0 = _mm256_mul_epu32(t0, t4);
                t1 = _mm256_mul_epu32(t1, t5);
                t2 = _mm256_mul_epu32(t2, t6);
                t3 = _mm256_mul_epu32(t3, t7);
                sums0 = _mm256_add_epi64(sums0, t0);
                sums1 = _mm256_add_epi64(sums1, t1);
                sums2 = _mm256_add_epi64(sums2, t2);
                sums3 = _mm256_add_epi64(sums3, t3);
            }
            msg_ = &msg_[64..];
            key_ = &key_[16..];
            remaining -= 64;
        }
        assert_eq!(remaining, 0);

        t0 = _mm256_unpacklo_epi64(sums0, sums1);
        t1 = _mm256_unpacklo_epi64(sums0, sums1);
        t2 = _mm256_unpacklo_epi64(sums2, sums3);
        t3 = _mm256_unpacklo_epi64(sums2, sums3);

        t4 = _mm256_inserti128_si256(t0, _mm256_castsi256_si128(t2), 0x1);
        t5 = _mm256_inserti128_si256(t1, _mm256_castsi256_si128(t3), 0x1);
        t0 = _mm256_permute2x128_si256(t0, t2, 0x31);
        t1 = _mm256_permute2x128_si256(t1, t3, 0x31);

        t4 = _mm256_add_epi64(t4, t5);
        t0 = _mm256_add_epi64(t0, t1);
        t0 = _mm256_add_epi64(t0, t4);

        let idx = out.len();
        out.reserve(32);
        out.set_len(out.len() + 32);
        let addr = out.as_mut_ptr().add(idx);
        _mm256_storeu_si256(addr as *mut __m256i, t0);
    }
}

pub fn new(key: &[u8]) -> Hash {
    debug_assert!(key.len() == NH_KEY_BYTES_PER_MESSAGE);
    let mut key_u32 = [0u32; NH_KEYS_PER_MESSAGE];
    for i in 0..NH_KEYS_PER_MESSAGE {
        key_u32[i] = LittleEndian::read_u32(&key[i * 4..]);
    }
    Hash { key: key_u32 }
}

#[test]
fn basic_small() {
    let key = vec![1; NH_KEY_BYTES_PER_MESSAGE];
    let h = new(&key);
    let msg = vec![0; 64];
    let mut out = Vec::new();
    h.hash(&mut out, &msg);
    assert_eq!(
        out,
        [
            8, 16, 24, 32, 24, 16, 8, 0, 8, 16, 24, 32, 24, 16, 8, 0, 8, 16, 24, 32, 24, 16, 8, 0,
            8, 16, 24, 32, 24, 16, 8, 0
        ]
    );
}
