use pqc_kyber::*;
use std::time::{Instant};

use crate::time_record::TimeRecord;

pub fn kyber_bench() -> TimeRecord{
    let mut time_holder = TimeRecord::default();
    let mut rng = rand::thread_rng();

    let mut start = Instant::now();
    let keys_bob = keypair(&mut rng);
    time_holder.key_gen = start.elapsed().as_micros();

    start = Instant::now();
    let (ciphertext, shared_secret_alice) = encapsulate(&keys_bob.public, &mut rng).ok().unwrap();
    time_holder.encap = start.elapsed().as_micros();

    start = Instant::now();
    let shared_secret_bob = decapsulate(&ciphertext, &keys_bob.secret).ok().unwrap();
    time_holder.decap = start.elapsed().as_micros();

    assert_eq!(shared_secret_alice, shared_secret_bob);

    time_holder
}
