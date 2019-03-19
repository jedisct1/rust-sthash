use super::{nh, poly1305};

pub const NHPOLY_KEY_BYTES: usize = poly1305::POLY_KEY_BYTES + nh::NH_KEY_BYTES_PER_MESSAGE;

const NHPOLY_HASHES_PER_POLY: usize = 16; // 16 * (4 * u64 sums) polys over 512 bytes

pub struct Hasher<'t> {
    key: &'t [u8],
}

impl<'t> Hasher<'t> {
    pub fn hash(&self, out: &mut [u8; 16], msg: &[u8]) {
        let key = self.key;
        let mut poly_key = [0u8; 16];
        poly_key.copy_from_slice(&key[0..16]);
        let nh_key = &key[16..];
        let mut nh_out = vec![];
        let mut remaining = msg.len();
        let mut off = 0;
        let mut st_poly = poly1305::new(poly_key);
        let st_nh = nh::new(nh_key);
        while remaining > nh::NH_MESSAGE_BYTES {
            st_nh.hash_avx2(&mut nh_out, &msg[off..off + nh::NH_MESSAGE_BYTES]);
            if nh_out.len() == nh::NH_OUTPUT_BYTES * NHPOLY_HASHES_PER_POLY {
                st_poly.update(&nh_out);
                nh_out.truncate(0);
            }
            off += nh::NH_MESSAGE_BYTES;
            remaining -= nh::NH_MESSAGE_BYTES;
        }
        if remaining > 0 {
            let mask = nh::NH_MESSAGE_UNIT - 1;
            let padded_len = (remaining + mask) & !mask;
            let mut unit = [0u8; nh::NH_MESSAGE_UNIT];
            let padded = &mut unit[..padded_len];
            padded[..remaining].copy_from_slice(&msg[off..]);
            st_nh.hash(&mut nh_out, &padded);
        }
        if !nh_out.is_empty() {
            st_poly.update(&nh_out);
        }
        st_poly.finalize_noadd(out);
    }
}

pub fn new(key: &[u8]) -> Hasher<'_> {
    assert_eq!(key.len(), NHPOLY_KEY_BYTES);
    Hasher { key }
}
