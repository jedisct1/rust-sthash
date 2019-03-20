# STHash

[![docs.rs](https://docs.rs/sthash/badge.svg)](https://docs.rs/sthash)

STHash is a fast, keyed, cryptographic hash function designed to process large, possibly untrusted data.

The flipside is that using a secret key (or, in this implementation, a secret seed) is mandatory. This is not as a general-purpose hash function.

A typical use of STHash is to compute keys for locally cached objects.

The construction relies on:

- A composition of two ϵ-almost-∆-universal functions, NH and Poly1305. See the [Adiantum](https://tosc.iacr.org/index.php/ToSC/article/view/7360/6530) paper for a justification of this composition.
- The KMAC keyed hash function, both to produce the final tag and as a XOF to derive the NH, Poly1305 and finalization keys.

The current code is portable, written in safe Rust, and has a lot of room for optimization.

However, it is already consistently faster than optimized BLAKE2bp implementations (using the `blake2b-simd` crate) on all platforms.

You can expect future versions to be even faster.

## Usage

```rust
use sthash::*;
use rand::{thread_rng, RngCore};

// This must be a random, secret seed.
let seed = [u8; SEED_BYTES];
thread_rng().fill_bytes(&mut seed);

// The key constructor accepts an optional application name
// Different personalization strings produce different keys
// from the same `seed`.
let key = Key::from_seed(&seed, Some(b"Documentation example"));

// Another personalization string, such as the purpose of the
// `Hasher`, can be provided here as well.
let hasher = Hasher::new(key, None);

// Returns a 256-bit hash.
let h1 = hasher.hash(data);

// `Hasher` structures can safely be reused to hash more data.
let h2 = hasher.hash(data2);
```

## Benchmarks

Measurements from the built-in benchmark, hashing 1 Mb data. Rust 1.33.
Get your own data with the `cargo bench` command.

| Machine                                 | BLAKE2bp (μs) | STHash (μs) | Ratio |
| --------------------------------------- | ------------- | ----------- | ----- |
| Core i9 2.9Ghz, MacOS                   | 391           | 95          | 4.1   |
| Xeon CPU E3-1245 V2 3.40GHz, OpenBSD VM | 2681          | 493         | 5.4   |
| ARMv7 (Scaleway C1), Linux              | 29402         | 7871        | 3.7   |
| Raspberry Pi 3b                         | 1             | 1           | 1     |

## References

- [UMAC: Fast and Secure Message Authentication](https://fastcrypto.org/umac/umac_proc.pdf) (J. Black, S.Halevi, H.Krawczyk, T.Krovetz, and P. Rogaway)
- [The Poly1305-AES message authentication code](https://cr.yp.to/mac/poly1305-20050329.pdf) (Daniel J. Bernstein)
- [Adiantum: length-preserving encryption for entry-level processors](https://tosc.iacr.org/index.php/ToSC/article/view/7360/6530) (Paul Crowley and Eric Biggers)
- [Short-output universal hash functions andtheir use in fast and secure data authentication](https://eprint.iacr.org/2011/116.pdf) (Yannick Seurin)

## Thanks

This crate is based on work by Paul Crowley and Eric Biggers.