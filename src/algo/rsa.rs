#[cfg(test)]
mod unit_test;
use num_bigint::{BigInt, BigUint, ToBigUint};
use crate::utils::{
    print_rsa, 
    gen_prime,
    gen_rand_inverses_below,
    convert_bigint_to_biguint_euclid_algo,
};
use std::thread;

#[derive(Clone)]
pub struct RSA {
    pub public_key: (BigUint, BigUint),
    private_key: (BigInt, BigUint) 
}

impl RSA {
    pub fn new() -> Self {
        let keys = Self::gen_keys(256, 3);
        Self {
            public_key: keys.0,
            private_key: keys.1,
        }
    }
    pub fn init(bit_size: u64, k_mil_rab: usize) -> Self{
        let keys = Self::gen_keys(bit_size, k_mil_rab);
        Self {
            public_key: keys.0,
            private_key: keys.1,
        }
    }
    pub fn gen_keys(bit_size: u64, k_mil_rab: usize) -> ((BigUint, BigUint), (BigInt, BigUint)){

        println!("\nGenerating the prime numbers ...");
        let handle = thread::spawn(move || {
            (gen_prime(bit_size, k_mil_rab), gen_prime(bit_size, k_mil_rab))
            }
        );
        print_rsa(bit_size);
        let (p,q) = handle.join().unwrap();
        let n = &p * &q;    
    
        let one = 1.to_biguint().unwrap();
        let euler_ind = (&p-&one) * (&q-&one);
        let (e, d) = gen_rand_inverses_below(&euler_ind);
    
        // public key = (n,e), private key = (d, Euler_indicator(n)) 
        // Nota Bene: In theory Euler_indicator is only needed to generate e,d
        // but for pratical reasons (in the function `decipher`), we keep it as part
        // of the private key
        ((n, e) , (d, euler_ind))
    }

    pub fn sign(&self, msg: &BigUint) -> BigUint{
        // signs the message msg
        let private_key = convert_bigint_to_biguint_euclid_algo(
            &self.private_key.0, 
            &self.private_key.1
        );
        msg.modpow(&private_key, &self.public_key.0)
    }

    pub fn decrypt_sign(&self, sig: &BigUint) -> BigUint{
        // decrypts a signature
        sig.modpow(&self.public_key.1, &self.public_key.0)
    }

}