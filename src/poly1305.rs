use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug)]
struct Key {
    r0: u64,
    r1: u64,
    r2: u64,
}

#[derive(Debug, Default)]
struct State {
    h0: u64,
    h1: u64,
    h2: u64,
}

impl Key {
    pub fn new(key: [u8; 16]) -> Self {
        let t0 = LittleEndian::read_u64(&key[0..8]);
        let t1 = LittleEndian::read_u64(&key[8..16]);

        let r0 = t0 & 0xffc0fffffff;
        let r1 = ((t0 >> 44) | (t1 << 20)) & 0xfffffc0ffff;
        let r2 = (t1 >> 24) & 0x00ffffffc0f;

        Key { r0, r1, r2 }
    }
}

pub struct Hasher {
    key: Key,
    st: State,
}

impl Hasher {
    pub fn update(&mut self, msg: &[u8]) {
        let (r0, r1, r2) = (self.key.r0, self.key.r1, self.key.r2);
        let (mut h0, mut h1, mut h2) = (self.st.h0, self.st.h1, self.st.h2);
        let s1 = r1.wrapping_mul(5 << 2);
        let s2 = r2.wrapping_mul(5 << 2);
        let hibit = 1u64 << 40;
        let mut remaining = msg.len();
        let mut cursor = Cursor::new(msg);

        while remaining > 0 {
            let t0 = cursor.read_u64::<LittleEndian>().unwrap();
            let t1 = cursor.read_u64::<LittleEndian>().unwrap();

            h0 += t0 & 0xfffffffffff;
            h1 += ((t0 >> 44) | (t1 << 20)) & 0xfffffffffff;
            h2 += ((t1 >> 24) & 0x3ffffffffff) | hibit;

            let d0 = (h0 as u128).wrapping_mul(r0 as u128)
                + (h1 as u128).wrapping_mul(s2 as u128)
                + (h2 as u128).wrapping_mul(s1 as u128);
            let mut d1 = (h0 as u128) * (r1 as u128)
                + (h1 as u128).wrapping_mul(r0 as u128)
                + (h2 as u128).wrapping_mul(s2 as u128);
            let mut d2 = (h0 as u128) * (r2 as u128)
                + (h1 as u128).wrapping_mul(r1 as u128)
                + (h2 as u128).wrapping_mul(r0 as u128);

            let c = d0 >> 44;
            h0 = d0 as u64 & 0xfffffffffff;
            d1 = d1.wrapping_add(c);
            let c = d1 >> 44;
            h1 = d1 as u64 & 0xfffffffffff;
            d2 = d2.wrapping_add(c);
            let c = d2 >> 42;
            h2 = d2 as u64 & 0x3ffffffffff;
            h0 = h0.wrapping_add((c as u64).wrapping_mul(5));
            let c = h0 >> 44;
            h0 &= 0xfffffffffff;
            h1 = h1.wrapping_add(c);
            remaining -= 16;
        }
        self.st.h0 = h0;
        self.st.h1 = h1;
        self.st.h2 = h2;
    }

    pub fn finalize_noadd(self, out: &mut [u8; 16]) {
        let (mut h0, mut h1, mut h2) = (self.st.h0, self.st.h1, self.st.h2);

        // carry h
        let c = h1 >> 44;
        h1 &= 0xfffffffffff;
        h2 += c;
        let c = h2 >> 42;
        h2 &= 0x3ffffffffff;
        h0 += c * 5;
        let c = h0 >> 44;
        h0 &= 0xfffffffffff;
        h1 += c;
        let c = h1 >> 44;
        h1 &= 0xfffffffffff;
        h2 += c;
        let c = h2 >> 42;
        h2 &= 0x3ffffffffff;
        h0 += c * 5;
        let c = h0 >> 44;
        h0 &= 0xfffffffffff;
        h1 += c;

        // compute h + (-p)
        let mut g0 = h0 + 5;
        let c = g0 >> 44;
        g0 &= 0xfffffffffff;
        let mut g1 = h1 + c;
        let c = g1 >> 44;
        g1 &= 0xfffffffffff;
        let mut g2 = (h2 + c).wrapping_sub(1u64 << 42);

        // select h if h < p, or h + (-p) if h >= p
        let mut mask = g2 >> 63;
        g0 &= mask;
        g1 &= mask;
        g2 &= mask;
        mask = !mask;
        h0 = (h0 & mask) | g0;
        h1 = (h1 & mask) | g1;
        h2 = (h2 & mask) | g2;

        // mac = h % (2^128)
        h0 |= h1 << 44;
        h1 = (h1 >> 20) | (h2 << 24);

        LittleEndian::write_u64(&mut out[0..8], h0);
        LittleEndian::write_u64(&mut out[8..16], h1);
    }

    #[allow(dead_code)]
    pub fn hash(out: &mut [u8; 16], key: [u8; 16], msg: &[u8]) {
        let mut h = new(key);
        h.update(msg);
        h.finalize_noadd(out);
    }
}

pub fn new(key: [u8; 16]) -> Hasher {
    let key = Key::new(key);
    let st = State::default();
    Hasher { key, st }
}
