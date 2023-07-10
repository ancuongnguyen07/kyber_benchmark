use pqcrypto_kyber::*;
use std::time::{Duration, Instant};

pub struct TimeRecord {
    key_gen: u128,
    encap: u128,
    decap: u128
}

impl Default for TimeRecord {
    fn default() -> Self {
        TimeRecord { key_gen: 0, encap: 0, decap: 0 }
    }
}

pub fn kyber512_bench() -> TimeRecord {
    let mut time_holder = TimeRecord::default();

    let mut start = Instant::now();
    let (pk, sk) = pqcrypto_kyber::kyber512_keypair();
    time_holder.key_gen = start.elapsed().as_millis();

    start = Instant::now();
    let (ss1, ct) = pqcrypto_kyber::kyber512_encapsulate(&pk);
    time_holder.encap = start.elapsed().as_millis();

    start = Instant::now();
    let ss2 = pqcrypto_kyber::kyber512_decapsulate(&ct, &sk);
    time_holder.decap = start.elapsed().as_millis();

    assert!(ss1 == ss2);

    time_holder
}