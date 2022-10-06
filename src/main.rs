mod utils;
use utils::{gen_prime};
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

    println!("Generating the prime numbers ...");
    let handle = thread::spawn(move || {
        (gen_prime(bit_size, k_mil_rab), gen_prime(bit_size, k_mil_rab))
        }
    );
    let (p,q) = handle.join().unwrap();    
    println!("p = {} \nq = {} \nn = {}", p.clone(), q.clone(), p.clone() * q.clone());
}
