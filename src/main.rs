mod utils;
use utils::{gen_prime, gen_rand_inverses_below, print_rsa, naive_hex_from_biguint};
use num_bigint::{ToBigUint};
use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    /// Number of bits of the numbers to generate
    #[arg(short, long, default_value_t = 512)]
    bit_size: u64,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: u32,
}


fn main(){
    let args = Args::parse();
    // println!("");
    // println!("Code launched with the following arguments {:#?}", args);

    let bit_size: u64= args.bit_size;
    let k_mil_rab: u32 = args.k_mil_rab;

    println!("\nGenerating the prime numbers ...");
    let handle = thread::spawn(move || {
        (gen_prime(bit_size, k_mil_rab), gen_prime(bit_size, k_mil_rab))
        }
    );
    let (p,q) = handle.join().unwrap();    

    let one = 1.to_biguint().unwrap();
    let euler_ind = (p.clone()-one.clone()) * (q.clone()-one.clone());
    let (e, _) = gen_rand_inverses_below(euler_ind);

    print_rsa(bit_size);
    let n = p*q;
    println!("\nYour public key in decimal is the pair [ \nn = {}  \n\ne = {} \n]", n, e);

    let n = naive_hex_from_biguint(n);
    let e = naive_hex_from_biguint(e);
    println!("\nYour public key in hexadecimal is the pair [ \nn = 0x{}  \n\ne = 0x{} \n]", n, e);

}
