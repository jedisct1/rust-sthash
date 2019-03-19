//! STHash - A fast cryptopgraphic hash function for large inputs.
//!
//! Note: this is *not* a replacement for a generic hash function, as a
//! secret seed is mandatory to protect against forgeries.
//!
//! ```rust
//! use rand::{thread_rng, RngCore};
//! use sthash::*;
//!
//! let mut seed = [0; SEED_BYTES];
//! thread_rng().fill_bytes(&mut seed);
//!
//! let key = Key::from_seed(&seed, Some(b"Application name"));
//! let hasher = Hasher::new(key, None);
//!
//! let h1 = hasher.hash(b"test data 1");
//! let h2 = hasher.hash(b"test data 2");
//! ```

#![allow(clippy::cast_lossless)]
#![allow(clippy::unreadable_literal)]

mod nh;
mod nhpoly1305;
mod poly1305;
mod sthash;

#[cfg(test)]
mod test;

pub use crate::sthash::*;
