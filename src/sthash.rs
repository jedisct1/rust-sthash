use super::nhpoly1305;
use byteorder::{ByteOrder, LittleEndian};
use sp800_185::{CShake, KMac};

pub const KEY_BYTES: usize = 32 + nhpoly1305::KEY_BYTES;

#[derive(Clone)]
pub struct Hash<'t> {
    nhpoly_key: &'t [u8],
    st_kmac: KMac,
}

impl<'t> Hash<'t> {
    pub fn hash(&self, msg: &[u8]) -> Vec<u8> {
        let st_nhpoly = nhpoly1305::new(self.nhpoly_key);
        let mut poly = [0u8; 16];
        st_nhpoly.hash(&mut poly, &msg);

        let mut msg_len_u8 = [0u8; 8];
        LittleEndian::write_u64(&mut msg_len_u8, msg.len() as u64);

        let mut st_kmac = self.st_kmac.clone();
        st_kmac.update(&msg_len_u8);
        st_kmac.update(&poly);
        let mut h = vec![0u8; 32];
        st_kmac.finalize(&mut h);
        h
    }
}

pub fn new<'t>(key: &'t [u8], personalization: &[u8]) -> Hash<'t> {
    if key.len() != KEY_BYTES {
        panic!("Incorrect key size");
    }
    let kmac_key = &key[..32];
    let nhpoly_key = &key[32..];
    let st_kmac = KMac::new_kmac128(kmac_key, personalization);
    Hash {
        nhpoly_key,
        st_kmac,
    }
}

pub fn extend_key(key: &[u8]) -> Vec<u8> {
    let mut st_cshake = CShake::new_cshake128(b"sthash key", &[]);
    st_cshake.update(key);
    let mut key1 = vec![0; KEY_BYTES];
    st_cshake.finalize(&mut key1);
    key1
}
