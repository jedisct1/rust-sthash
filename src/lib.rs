#![doc = include_str!("../README.md")]
#![allow(clippy::cast_lossless)]
#![allow(clippy::unreadable_literal)]

mod nhpoly1305;
mod poly1305;
mod sthash;

#[cfg(test)]
mod test;

pub use crate::sthash::*;
