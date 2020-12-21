use blake2b_simd;
use blake2b_simd::blake2bp;
use criterion::{criterion_group, criterion_main, Criterion};
use hmac::{Hmac, Mac, NewMac};
use sha2::{Sha256, Sha512};
use sthash::*;
use {hmac, sha2};

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
    mac.update(msg);
    mac.finalize().into_bytes().to_vec()
}

fn hash_sha256(msg: &[u8]) -> Vec<u8> {
    let mut mac = Hmac::<Sha256>::new_varkey(b"key").unwrap();
    mac.update(msg);
    mac.finalize().into_bytes().to_vec();
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

    c.bench_function("HMAC-SHA256 1 Mo", |b| {
        let msg = vec![0x69; 1_000_000];
        b.iter(|| hash_sha256(&msg))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
