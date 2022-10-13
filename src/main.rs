mod utils;
use utils::{gen_prime, gen_rand_inverses_below, print_rsa};
use utils::{naive_hex_from_biguint, encrypt, decrypt};
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
    let (e, d) = gen_rand_inverses_below(euler_ind.clone());

    print_rsa(bit_size);
    let n = p*q;
    println!("\nThe public key in decimal is the pair [ \nn = {}  \n\ne = {} \n]", n, e);

    let hex_n = naive_hex_from_biguint(n.clone());
    let hex_e = naive_hex_from_biguint(e.clone());
    println!("\nThe public key in hexadecimal is the pair [ \nn = 0x{}  \n\ne = 0x{} \n]", hex_n, hex_e);

    loop {
        let mut msg = String::new();

        println!("\nPlease enter a message to encrypt, press Ctrl + C to exit");

        io::stdin()
            .read_line(&mut msg)
            .expect("Failed to read line");

        // ciphering
        let cipher = encrypt(msg.clone(), e.clone(), n.clone());
        println!("\nHere is the cipher of your message:");
        println!("{cipher}");

        // deciphering
        println!("\nHere is the deciphered message:");
        let decipher = decrypt(cipher, d.clone(), euler_ind.clone(), n.clone());
        println!("{decipher}");
        
 
    
    }
    // let s = String::from("Hello world!");
    // let ss = String::new();
    // println!("{s}");

    // let c = s.as_bytes().to_vec();
    // for a in &c{
    //     // ss.push(a);
    //     println!("{a}");
    // }

    // use num_bigint::BigUint;

    // let bb: String = s.as_bytes()
    //     .to_vec()
    //     .iter()
    //     .map(ToString::to_string)
    //     .collect();

    // let aa = bb.parse::<BigUint>().expect("Failed parse to BigUint");

    // println!("{aa}");

    // let text = "hello world!";
    // let ch : char = text.chars().nth(5).unwrap();
    // println!("{ch}");
    // println!("{}", ch == ' ');
    // println!("{}", text.len());


    // assert_eq!(&[104, 101, 108, 108, 111], s.as_bytes());
}

