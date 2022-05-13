#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use super::*;

impl Hasher {
    #[allow(clippy::cast_ptr_alignment)]
    #[target_feature(enable = "avx2")]
    #[inline]
    pub(crate) unsafe fn hash_avx2(&self, out: &mut Vec<u8>, msg: &[u8]) {
        let mut key_ = &self.key[..];
        let mut k0 = _mm256_loadu_si256(key_.as_ptr().add(0) as *const __m256i);
        let mut k1 = _mm256_loadu_si256(key_.as_ptr().add(4) as *const __m256i);
        let mut k2;
        let mut k3;
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
            let t3 = _mm256_loadu_si256(msg_.as_ptr().add(0) as *const __m256i);
            k2 = _mm256_loadu_si256(key_.as_ptr().add(0) as *const __m256i);
            k3 = _mm256_loadu_si256(key_.as_ptr().add(4) as *const __m256i);
            {
                let t0 = _mm256_add_epi32(k0, t3);
                let t1 = _mm256_add_epi32(k1, t3);
                let t2 = _mm256_add_epi32(k2, t3);
                let t3 = _mm256_add_epi32(k3, t3);
                let t4 = _mm256_shuffle_epi32(t0, 0x10);
                let t0 = _mm256_shuffle_epi32(t0, 0x32);
                let t5 = _mm256_shuffle_epi32(t1, 0x10);
                let t1 = _mm256_shuffle_epi32(t1, 0x32);
                let t6 = _mm256_shuffle_epi32(t2, 0x10);
                let t2 = _mm256_shuffle_epi32(t2, 0x32);
                let t7 = _mm256_shuffle_epi32(t3, 0x10);
                let t3 = _mm256_shuffle_epi32(t3, 0x32);
                let t0 = _mm256_mul_epu32(t0, t4);
                let t1 = _mm256_mul_epu32(t1, t5);
                let t2 = _mm256_mul_epu32(t2, t6);
                let t3 = _mm256_mul_epu32(t3, t7);
                sums0 = _mm256_add_epi64(sums0, t0);
                sums1 = _mm256_add_epi64(sums1, t1);
                sums2 = _mm256_add_epi64(sums2, t2);
                sums3 = _mm256_add_epi64(sums3, t3);
            }
            let t3 = _mm256_loadu_si256(msg_.as_ptr().add(32) as *const __m256i);
            k0 = _mm256_loadu_si256(key_.as_ptr().add(8) as *const __m256i);
            k1 = _mm256_loadu_si256(key_.as_ptr().add(12) as *const __m256i);
            {
                let t0 = _mm256_add_epi32(k2, t3);
                let t1 = _mm256_add_epi32(k3, t3);
                let t2 = _mm256_add_epi32(k0, t3);
                let t3 = _mm256_add_epi32(k1, t3);
                let t4 = _mm256_shuffle_epi32(t0, 0x10);
                let t0 = _mm256_shuffle_epi32(t0, 0x32);
                let t5 = _mm256_shuffle_epi32(t1, 0x10);
                let t1 = _mm256_shuffle_epi32(t1, 0x32);
                let t6 = _mm256_shuffle_epi32(t2, 0x10);
                let t2 = _mm256_shuffle_epi32(t2, 0x32);
                let t7 = _mm256_shuffle_epi32(t3, 0x10);
                let t3 = _mm256_shuffle_epi32(t3, 0x32);
                let t0 = _mm256_mul_epu32(t0, t4);
                let t1 = _mm256_mul_epu32(t1, t5);
                let t2 = _mm256_mul_epu32(t2, t6);
                let t3 = _mm256_mul_epu32(t3, t7);
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

        let t0 = _mm256_unpacklo_epi64(sums0, sums1);
        let t1 = _mm256_unpackhi_epi64(sums0, sums1);
        let t2 = _mm256_unpacklo_epi64(sums2, sums3);
        let t3 = _mm256_unpackhi_epi64(sums2, sums3);

        let t4 = _mm256_inserti128_si256(t0, _mm256_castsi256_si128(t2), 0x1);
        let t5 = _mm256_inserti128_si256(t1, _mm256_castsi256_si128(t3), 0x1);
        let t0 = _mm256_permute2x128_si256(t0, t2, 0x31);
        let t1 = _mm256_permute2x128_si256(t1, t3, 0x31);

        let t4 = _mm256_add_epi64(t4, t5);
        let t0 = _mm256_add_epi64(t0, t1);
        let t0 = _mm256_add_epi64(t0, t4);

        let idx = out.len();
        out.reserve(32);
        out.set_len(out.len() + 32);
        let addr = out.as_mut_ptr().add(idx);
        _mm256_storeu_si256(addr as *mut __m256i, t0);
    }
}
