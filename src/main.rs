mod utils;
use utils::{gen_prime, gen_rand_inverses_below};
use num_bigint::BigUint;
use num_traits::One;
use clap::Parser;
use std::thread;

#[derive(Parser, Debug)]
struct Args {
    /// Number of bits of the numbers to generate
    #[arg(short, long, default_value_t = 500)]
    bit_size: u64,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: u32,
}


fn main(){
    let args = Args::parse();
    println!("");
    println!("Code launched with the following arguments {:#?}", args);

    let bit_size: u64= args.bit_size;
    let k_mil_rab: u32 = args.k_mil_rab;

    println!("\nGenerating the prime numbers ...");
    let handle = thread::spawn(move || {
        (gen_prime(bit_size, k_mil_rab), gen_prime(bit_size, k_mil_rab))
        }
    );
    let (p,q) = handle.join().unwrap();    

    let one : BigUint = One::one();
    let euler_ind = (p.clone()-one.clone()) * (q.clone()-one.clone());
    let (e, _) = gen_rand_inverses_below(euler_ind);

    println!("\nYour public key is the pair [ \nn = {}  \n\ne = {} \n]", p*q, e);
}
