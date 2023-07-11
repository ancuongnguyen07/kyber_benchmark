use std::time::{Instant};

use crate::time_record::TimeRecord;

pub fn kyber512_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber512_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber512_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber512_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()
}

pub fn kyber512_90s_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber51290s_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber51290s_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber51290s_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()

}

pub fn kyber768_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber768_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber768_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber768_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()

}

pub fn kyber76890s_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber76890s_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber76890s_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber76890s_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()

}

pub fn kyber1024_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber1024_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber1024_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber1024_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()

}

pub fn kyber102490s_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber102490s_keypair();
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber102490s_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber102490s_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_micros();

    assert!(ss1 == ss2);

    time_holder.clone()

}