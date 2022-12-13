use std::sync::Arc;

use byteorder::{ByteOrder, LittleEndian};
use tiny_keccak::{CShake, Hasher as _, Kmac};

use super::nhpoly1305;

const KMAC_KEY_BYTES: usize = 32;
const KEY_BYTES: usize = KMAC_KEY_BYTES + nhpoly1305::NHPOLY_KEY_BYTES;

/// Hash output size, in bytes
pub const OUTPUT_BYTES: usize = 32;

/// Recommended seed size, in bytes
pub const SEED_BYTES: usize = 32;

/// Minimum seed size, in bytes
pub const MIN_SEED_BYTES: usize = 16;

/// A large secret key, derived from a secret seed
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key(Vec<u8>);

struct HashInner {
    key: Key,
    st_kmac: Kmac,
}

/// A `Hasher` can be reused to compute multiple hashes using the same key
#[derive(Clone)]
pub struct Hasher {
    inner: Arc<HashInner>,
}

impl Hasher {
    /// Returns an `OUTPUT_BYTES` hash of the message
    pub fn hash(&self, msg: &[u8]) -> Vec<u8> {
        let nhpoly_key = &self.inner.key.0[32..];
        debug_assert_eq!(nhpoly_key.len(), nhpoly1305::NHPOLY_KEY_BYTES);
        let st_nhpoly = nhpoly1305::Hasher::new(nhpoly_key);
        let mut poly = [0u8; 16];
        st_nhpoly.hash(&mut poly, msg);

        let mut msg_len_u8 = [0u8; 8];
        LittleEndian::write_u64(&mut msg_len_u8, msg.len() as u64);

        let mut st_kmac = self.inner.st_kmac.clone();
        st_kmac.update(&msg_len_u8);
        st_kmac.update(&poly);
        let mut h = vec![0u8; 32];
        st_kmac.finalize(&mut h);
        h
    }

    /// Creates a new `Hasher` object using `key`
    ///
    /// `personalization` is an optional context that describes the purpose
    /// of the hashes this `Hasher` will compute.
    /// The same key used with the same messages, but in different contexts will
    /// produce different outputs.
    pub fn new(key: Key, personalization: Option<&[u8]>) -> Hasher {
        debug_assert_eq!(key.0.len(), KEY_BYTES);
        let kmac_key = &key.0[..KMAC_KEY_BYTES];
        let st_kmac = Kmac::v128(kmac_key, personalization.unwrap_or_default());
        Hasher {
            inner: Arc::new(HashInner { key, st_kmac }),
        }
    }
}

impl Key {
    /// Creates a new key from a secret `seed`
    ///
    /// This expands the `seed` into a large secret key.
    /// `personalization` is an optional context, that can be set to the
    /// application name. The same `seed` used in different contexts will
    /// produce different keys, hence different hashes.
    pub fn from_seed(seed: &[u8], personalization: Option<&[u8]>) -> Key {
        if seed.len() < MIN_SEED_BYTES {
            panic!("Seed is too short");
        }
        let mut st_cshake = CShake::v128(b"sthash key", personalization.unwrap_or_default());
        st_cshake.update(seed);
        let mut key = vec![0; KEY_BYTES];
        st_cshake.finalize(&mut key);
        Key(key)
    }
}
