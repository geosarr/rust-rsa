use crate::utils::{gen_prime, gen_rand_inverses_below, print_rsa};
use num_bigint::{BigInt, BigUint, ToBigUint};
use std::thread;

pub struct PrimeGenerator {}

impl PrimeGenerator {
    pub fn gen_keys(bit_size: u64, k_mil_rab: usize) -> ((BigUint, BigUint), (BigInt, BigUint)) {
        println!("\nGenerating the prime numbers ...");
        let handle = thread::spawn(move || {
            (
                gen_prime(bit_size, k_mil_rab),
                gen_prime(bit_size, k_mil_rab),
            )
        });
        print_rsa(bit_size);
        let (p, q) = handle.join().unwrap();
        let n = &p * &q;

        let one = 1.to_biguint().unwrap();
        let euler_ind = (&p - &one) * (&q - &one);
        let (e, d) = gen_rand_inverses_below(&euler_ind);

        // public key = (n,e), private key = (d, Euler_indicator(n))
        // Nota Bene: In theory Euler_indicator is only needed to generate e,d
        // but for pratical reasons (in the function `decipher`), we keep it as part
        // of the private key
        ((n, e), (d, euler_ind))
    }
}
