# STHash

STHash is a fast, keyed, cryptographic hash function designed to process large, possibly untrusted data.

The flipside is that using a secret key (or, in this implementation, a secret seed) is mandatory.

A typical use of STHash is to compute keys for locally cached objects.

The construction relies on:

- a composition of two ϵ-almost-∆-universal functions, NH and Poly1305. See the [Adiantum](https://tosc.iacr.org/index.php/ToSC/article/view/7360/6530) paper for a justification of this composition.
- The KMAC keyed hash function, both to produce the final tag and as a XOF to derive the NH, Poly1305 and finalization keys.

The current code is portable, written in safe Rust, and has a lot of room for optimization.

However, even without vectorization, it is already consistently faster than optimized BLAKE2bp implementations (using the `blake2b-simd` crate) on all platforms.

You can expect a 2x to 4x speed increase in future versions.

## Usage

```rust
use sthash::*;

// This must be a random, secret seed.
let seed: [u8; SEED_BYTES] = [...];

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

## References

- [UMAC: Fast and Secure Message Authentication](https://fastcrypto.org/umac/umac_proc.pdf)
- [The Poly1305-AES message authentication code](https://cr.yp.to/mac/poly1305-20050329.pdf)
- [Adiantum: length-preserving encryption for entry-level processors](https://tosc.iacr.org/index.php/ToSC/article/view/7360/6530)
- [Beyond-Birthday-Bound Secure MACs](http://materials.dagstuhl.de/files/18/18021/18021.YannickSeurin.Slides.pdf)
- [Short-output universal hash functions andtheir use in fast and secure data authentication](https://eprint.iacr.org/2011/116.pdf)
