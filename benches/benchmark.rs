#[macro_use]
extern crate criterion;
use blake2b_simd;
use {hmac, sha2};

use blake2b_simd::blake2bp;
use criterion::Criterion;
use hmac::{Hmac, Mac};
use sha2::Sha512;
use sthash::*;

fn hash(hasher: &Hasher, msg: &[u8]) -> Vec<u8> {
    hasher.hash(msg)
}

fn hash_blake2bp(msg: &[u8]) -> blake2b_simd::Hash {
    blake2bp::Params::new().to_state().update(msg).finalize()
}

fn hash_blake2b(msg: &[u8]) -> blake2b_simd::Hash {
    blake2b_simd::Params::new()
        .to_state()
        .update(msg)
        .finalize()
}

fn hash_sha512(msg: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha512>::new_varkey(b"key").unwrap();
    mac.input(msg);
    mac.result().code().as_slice().to_vec()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("STHash 1 Mo", |b| {
        let seed = [0x42; SEED_BYTES];
        let key = Key::from_seed(&seed, Some(b"test suite"));
        let hasher = Hasher::new(key, None);

        let msg = vec![0x69; 1_000_000];
        b.iter(|| hash(&hasher, &msg))
    });

    c.bench_function("BLAKE2bp 1 Mo", |b| {
        let msg = vec![0x69; 1_000_000];
        b.iter(|| hash_blake2bp(&msg))
    });

    c.bench_function("BLAKE2b 1 Mo", |b| {
        let msg = vec![0x69; 1_000_000];
        b.iter(|| hash_blake2b(&msg))
    });

    c.bench_function("HMAC-SHA512 1 Mo", |b| {
        let msg = vec![0x69; 1_000_000];
        b.iter(|| hash_sha512(&msg))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
