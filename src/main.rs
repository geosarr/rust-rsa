mod utils;
use utils::{gen_prime, gen_rand_inverses_below, print_rsa};
use utils::{naive_hex_from_biguint, encrypt, decrypt, gen_vigenere};
use num_bigint::{ToBigUint};
use clap::Parser;
use std::thread;
use std::string::String;
use std::io;

#[derive(Parser, Debug)]
struct Args {
    /// Number of bits of the numbers to generate
    #[arg(short, long, default_value_t = 512)]
    bit_size: u64,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: usize,

    /// Length of the vigenere key
    #[arg(short, long, default_value_t = 10)]
    vig_key_size: usize, 
}


fn main(){
    let args = Args::parse();
    // println!("");
    // println!("Code launched with the following arguments {:#?}", args);

    let bit_size: u64= args.bit_size;
    let k_mil_rab: usize = args.k_mil_rab;
    let vig_key_size: usize = args.vig_key_size;

    println!("\nGenerating the prime numbers ...");
    let handle = thread::spawn(move || {
        (gen_prime(bit_size, k_mil_rab), gen_prime(bit_size, k_mil_rab))
        }
    );
    let (p,q) = handle.join().unwrap();    

    let one = 1.to_biguint().unwrap();
    let euler_ind = (&p-&one) * (&q-&one);
    let (e, d) = gen_rand_inverses_below(&euler_ind);

    print_rsa(bit_size);
    let n = p*q;
    println!("\nThe public key in decimal is the pair 
    [ \nn = {}  \n\ne = {} \n]", n, e);

    let hex_n = naive_hex_from_biguint(&n);
    let hex_e = naive_hex_from_biguint(&e);
    println!("\nThe public key in hexadecimal is the pair 
    [ \nn = 0x{}  \n\ne = 0x{} \n]", hex_n, hex_e);

    let vigenere = gen_vigenere(&vig_key_size, &n);
    loop {
        let mut msg = String::new();

        println!("\nPlease enter a message to encrypt, press Ctrl + C to exit");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        // ciphering
        let (enc_vigenere, enc_msg) = encrypt(&msg, &vigenere, &e, &n);
        // println!("\nHere is the cipher of your message:");
        // println!("{:?},\n{:?}", enc_vigenere, enc_msg);

        // deciphering
        println!("\nHere is the deciphered message:");
        let deciphered_msg = decrypt((&enc_vigenere, &enc_msg), &d, &euler_ind, &n);
        println!("{}",deciphered_msg);
    
    }

}

