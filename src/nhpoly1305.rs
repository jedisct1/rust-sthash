use super::{nh, poly1305};

pub const KEY_BYTES: usize = 16 + 4096;

const NH_HASHES_PER_POLY: usize = 16; // 16 * (4 * u64 sums) polys over 512 bytes

pub struct Hash<'t> {
    key: &'t [u8],
}

impl<'t> Hash<'t> {
    pub fn hash(&self, out: &mut [u8; 16], msg: &[u8]) {
        let key = self.key;
        let mut poly_key = [0u8; 16];
        poly_key.copy_from_slice(&key[0..16]);
        let nh_key = &key[16..];
        let mut nh_out = vec![];
        let mut remaining = msg.len();
        let mut off = 0;
        let mut st_poly = poly1305::Hash::new(poly_key);
        let st_nh = nh::new(nh_key);
        while remaining > nh::NH_MESSAGE_BYTES {
            st_nh.hash(&mut nh_out, &msg[off..off + nh::NH_MESSAGE_BYTES]);
            if nh_out.len() == nh::NH_BYTES * NH_HASHES_PER_POLY {
                st_poly.update(&nh_out);
                nh_out.truncate(0);
            }
            off += nh::NH_MESSAGE_BYTES;
            remaining -= nh::NH_MESSAGE_BYTES;
        }
        if remaining > 0 {
            let mask = nh::NH_MESSAGE_UNIT - 1;
            let padded_len = (remaining + mask) & !mask;
            let mut padded = vec![0; padded_len];
            padded.copy_from_slice(&msg[off..]);
            st_nh.hash(&mut nh_out, &padded);
        }
        if !nh_out.is_empty() {
            st_poly.update(&nh_out);
        }
        st_poly.finalize_noadd(out);
    }
}

pub fn new(key: &[u8]) -> Hash<'_> {
    if key.len() != KEY_BYTES {
        panic!("Incorrect key size");
    }
    Hash { key }
}
