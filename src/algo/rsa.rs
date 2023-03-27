#[cfg(test)]
mod unit_test;
use crate::algo::PrimeGenerator;
use crate::utils::convert_bigint_to_biguint_euclid_algo;
use num_bigint::{BigInt, BigUint};

#[derive(Clone)]
pub struct RSA {
    pub public_key: (BigUint, BigUint),
    private_key: (BigInt, BigUint),
}

impl Default for RSA {
    fn default() -> Self {
        Self::new()
    }
}
impl RSA {
    pub fn new() -> Self {
        let keys = PrimeGenerator::gen_keys(256, 3);
        Self {
            public_key: keys.0,
            private_key: keys.1,
        }
    }
    pub fn init(bit_size: u64, k_mil_rab: usize) -> Self {
        let keys = PrimeGenerator::gen_keys(bit_size, k_mil_rab);
        Self {
            public_key: keys.0,
            private_key: keys.1,
        }
    }

    pub fn sign(&self, msg: &BigUint) -> BigUint {
        // signs the message msg
        let private_key =
            convert_bigint_to_biguint_euclid_algo(&self.private_key.0, &self.private_key.1);
        msg.modpow(&private_key, &self.public_key.0)
    }

    pub fn decrypt_sign(&self, sig: &BigUint) -> BigUint {
        // decrypts a signature
        sig.modpow(&self.public_key.1, &self.public_key.0)
    }
}
