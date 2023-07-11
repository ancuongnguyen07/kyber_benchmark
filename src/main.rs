use crate::time_record::TimeHolder;

mod crypto;
mod time_record;

const N: u128 = 1_000;

fn pqcrypto_kyber_bench() {
    let mut pqcrypto_time = TimeHolder::default();

    // PQCrypto_Kyber running
    println!("PQCrypto running!!!!");
    for _ in 0..N {
        pqcrypto_time.kyber_1024 += crypto::pqcrypto_kyber::kyber1024_bench();
        pqcrypto_time.kyber_1024_90s += crypto::pqcrypto_kyber::kyber102490s_bench();
        pqcrypto_time.kyber_768 += crypto::pqcrypto_kyber::kyber768_bench();
        pqcrypto_time.kyber_768_90s += crypto::pqcrypto_kyber::kyber76890s_bench();
        pqcrypto_time.kyber_512 += crypto::pqcrypto_kyber::kyber512_bench();
        pqcrypto_time.kyber_512_90s += crypto::pqcrypto_kyber::kyber512_90s_bench();
    }

    pqcrypto_time.kyber_1024.divide(N);
    pqcrypto_time.kyber_1024_90s.divide(N);
    pqcrypto_time.kyber_768.divide(N);
    pqcrypto_time.kyber_768_90s.divide(N);
    pqcrypto_time.kyber_512.divide(N);
    pqcrypto_time.kyber_512_90s.divide(N);

    println!("{}", pqcrypto_time);
}

fn main() {

}
