mod conversion;
mod design;
mod random_generator;
#[cfg(test)]
mod unit_tests;

pub use conversion::*;
pub use design::print_rsa;
pub use random_generator::*;

use crate::algo::PrimeGenerator;
use num_bigint::Sign::Plus;
use num_bigint::{BigInt, BigUint, ToBigInt, ToBigUint};
use std::fs::File;
use std::io;
use std::io::Write;

pub fn cli_keys(bit_size: u64, k_mil_rab: usize, output: String) {
    let ((n, e), (d, euler_ind)) = PrimeGenerator::gen_keys(bit_size, k_mil_rab);

    println!(
        "\nThe public key in decimal is the pair 
        [ \nn = {}  \n\ne = {} \n]",
        n, e
    );
    println!("\nThe private key in decimal is \nd = {}", d);

    let hex_d = convert_bigint_to_biguint_euclid_algo(&d, &euler_ind);
    let hex_d = naive_hex_from_biguint(&hex_d);
    let hex_n = naive_hex_from_biguint(&n);
    let hex_e = naive_hex_from_biguint(&e);

    println!(
        "\n\n\nThe public key in hexadecimal is the pair 
        [ \nn = 0x{}  \n\ne = 0x{} \n]",
        hex_n, hex_e
    );
    println!("\nThe private key in hexdecimal is \nd = 0x{}", hex_d);

    let mut private_key_path = output.clone();
    private_key_path.push_str("/rsa_key");
    let mut file_private =
        File::create(private_key_path).expect("Failed to create private key file");
    file_private
        .write_all(b"-----BEGIN RSA PRIVATE KEY-----")
        .expect("Failed to write in private key file");
    let len_line = 50;
    for i in 0..hex_d.len() / len_line {
        file_private
            .write_all(b"\n")
            .expect("Failed to write in private key file");
        file_private
            .write_all(hex_d[i * len_line..(i + 1) * len_line].as_bytes())
            .expect("Failed to write in private key file");
    }
    file_private
        .write_all(b"\n------END RSA PRIVATE KEY------")
        .expect("Failed to write in private key file");

    let mut public_key_path = output;
    public_key_path.push_str("/rsa_key.pub");
    let mut file_public = File::create(public_key_path).expect("Failed to create private key file");
    file_public
        .write_all(b"rsa ")
        .expect("Failed to write in private key file");
    let mut long_public = hex_n;
    long_public.push('/');
    long_public.push_str(&hex_e);
    file_public
        .write_all(long_public.as_bytes())
        .expect("Failed to write in private key file");
}

pub fn run_interactive(bit_size: u64, k_mil_rab: usize, vig_key_size: usize) {
    let ((n, e), (d, euler_ind)) = PrimeGenerator::gen_keys(bit_size, k_mil_rab);

    let vigenere = gen_vigenere(&vig_key_size, &n);
    loop {
        let mut msg = String::new();

        println!("\nPlease enter a message to encrypt, press Ctrl + C to exit");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        // ciphering
        let (enc_vigenere, enc_msg) = encrypt(&msg, &vigenere, &e, &n);
        println!("\nHere is the cipher of your message:");
        println!("{:?},\n{:?}", enc_vigenere, enc_msg);

        // deciphering
        println!("\nHere is the deciphered message:");
        let deciphered_msg = decrypt((&enc_vigenere, &enc_msg), &d, &euler_ind, &n);
        println!("{}", deciphered_msg);
    }
}

fn encrypt(
    msg: &String,
    vigenere: &Vec<BigUint>,
    e: &BigUint,
    n: &BigUint,
) -> (Vec<BigUint>, Vec<BigUint>) {
    // transform msg to vec
    let vec_msg: Vec<BigUint> = msg
        .as_bytes()
        .to_vec()
        .iter()
        .map(|x| x.to_biguint().unwrap())
        .collect();
    // encrypt msg with plain Vigenere key
    let enc_vec_msg = vec_msg
        .iter()
        .enumerate()
        .map(|(pos, x)| (x + &vigenere[pos % vigenere.len()]).modpow(e, n))
        .collect();
    // encrypt Vigenere key with public key (n,e)
    let enc_vigenere = vigenere.iter().map(|x| x.modpow(e, n)).collect();

    (enc_vigenere, enc_vec_msg)
}

fn decrypt(
    cipher: (&Vec<BigUint>, &Vec<BigUint>),
    d: &BigInt,
    euler_ind: &BigUint,
    n: &BigUint,
) -> String {
    let (enc_vigenere, enc_vec_msg) = &cipher;
    //  decrypt the encrypted Vigenere key
    let vigenere: Vec<BigUint> = enc_vigenere
        .iter()
        .map(|x| decipher(x, d, euler_ind, n))
        .collect();
    //  decrypt the msg with the plain Vigenere key
    let vec_msg: Vec<u8> = enc_vec_msg
        .iter()
        .enumerate()
        .map(|(pos, x)| decipher(x, d, euler_ind, n) - &vigenere[pos % vigenere.len()])
        .map(|x| x.to_string().parse::<u8>().unwrap())
        .collect();
    std::str::from_utf8(&vec_msg).unwrap().to_string()
}

fn decipher(cipher: &BigUint, d: &BigInt, euler_ind: &BigUint, n: &BigUint) -> BigUint {
    let one = 1.to_bigint().unwrap();
    let zero = 0.to_bigint().unwrap();
    // For now, the method modpow in struct BigInt does not
    // support negative exponents, hence we replace d by its remainder
    // (>= 0) wrt the Euler indicator when its is negative. It
    // works by definition of the remainder
    if d >= &zero {
        let _d = d.to_biguint().unwrap();
        cipher.modpow(&_d, n)
    } else {
        let _euler_ind = BigInt::from_biguint(Plus, euler_ind.clone());
        let remainder = d.modpow(&one, &_euler_ind).to_biguint().unwrap();
        cipher.modpow(&remainder, n)
    }
}
