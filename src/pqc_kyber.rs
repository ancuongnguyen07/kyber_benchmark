use pqc_kyber::*;
use std::time::{Duration, Instant};

fn temp() {
    let mut rng = rand::thread_rng();
    let keys_bob = keypair(&mut rng);
    let (ciphertext, shared_secret_alice) = encapsulate(&keys_bob.public,
                                                                            &mut rng).unwrap();
    let shared_secret_bob = decapsulate(&ciphertext,
                                                    &keys_bob.secret).unwrap();

    assert_eq!(shared_secret_alice, shared_secret_bob);
}

