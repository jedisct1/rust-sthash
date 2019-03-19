use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

pub const NH_NUM_PASSES: usize = 4;
pub const NH_PAIR_STRIDE: usize = 2;
pub const NH_MESSAGE_UNIT: usize = NH_PAIR_STRIDE * 8; // 16
pub const NH_NUM_STRIDES: usize = 64;
pub const NH_MESSAGE_DWORDS: usize = NH_PAIR_STRIDE * 2 * NH_NUM_STRIDES;
pub const NH_MESSAGE_BYTES: usize = NH_MESSAGE_DWORDS * 4; // 1Kb
pub const NH_BYTES: usize = 8 * 4;

pub struct Hash {
    key: Vec<u32>,
}

impl Hash {
    #[inline(always)]
    pub fn hash(&self, out: &mut Vec<u8>, msg: &[u8]) {
        let mut cursor = Cursor::new(msg);
        let mut remaining = msg.len();
        let (mut s0, mut s1, mut s2, mut s3) = (0u64, 0u64, 0u64, 0u64);
        let mut key_ = self.key.as_slice();

        debug_assert_eq!(NH_NUM_PASSES, 4);
        debug_assert_eq!(remaining % NH_MESSAGE_UNIT, 0);
        debug_assert!(key_.len() >= remaining / NH_MESSAGE_UNIT * 16);
        while remaining > 0 {
            let m0 = cursor.read_u32::<LittleEndian>().unwrap();
            let m1 = cursor.read_u32::<LittleEndian>().unwrap();
            let m2 = cursor.read_u32::<LittleEndian>().unwrap();
            let m3 = cursor.read_u32::<LittleEndian>().unwrap();

            s0 = s0.wrapping_add(
                ((m0.wrapping_add(key_[0])).wrapping_mul(m2.wrapping_add(key_[2]))) as u64,
            );
            s1 = s1.wrapping_add(
                ((m0.wrapping_add(key_[4])).wrapping_mul(m2.wrapping_add(key_[6]))) as u64,
            );
            s2 = s2.wrapping_add(
                ((m0.wrapping_add(key_[8])).wrapping_mul(m2.wrapping_add(key_[10]))) as u64,
            );
            s3 = s3.wrapping_add(
                ((m0.wrapping_add(key_[12])).wrapping_mul(m2.wrapping_add(key_[14]))) as u64,
            );
            s0 = s0.wrapping_add(
                ((m1.wrapping_add(key_[1])).wrapping_mul(m3.wrapping_add(key_[3]))) as u64,
            );
            s1 = s1.wrapping_add(
                ((m1.wrapping_add(key_[5])).wrapping_mul(m3.wrapping_add(key_[7]))) as u64,
            );
            s2 = s2.wrapping_add(
                ((m1.wrapping_add(key_[9])).wrapping_mul(m3.wrapping_add(key_[11]))) as u64,
            );
            s3 = s3.wrapping_add(
                ((m1.wrapping_add(key_[13])).wrapping_mul(m3.wrapping_add(key_[15]))) as u64,
            );

            key_ = &key_[NH_MESSAGE_UNIT / 4..];
            remaining -= NH_MESSAGE_UNIT;
        }
        out.write_u64::<LittleEndian>(s0).unwrap();
        out.write_u64::<LittleEndian>(s1).unwrap();
        out.write_u64::<LittleEndian>(s2).unwrap();
        out.write_u64::<LittleEndian>(s3).unwrap();
    }
}

pub fn new(key: &[u8]) -> Hash {
    let mut key_u32 = Vec::with_capacity(key.len() / 4);
    let mut cursor = Cursor::new(key);
    for _ in 0..key_u32.capacity() {
        key_u32.push(cursor.read_u32::<LittleEndian>().unwrap());
    }
    Hash { key: key_u32 }
}
