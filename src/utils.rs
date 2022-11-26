#[cfg(test)]
mod unit_tests;
use num_bigint::{RandBigInt, BigInt, BigUint, ToBigUint};
use num_bigint::{ToBigInt, Sign::Plus, Sign::Minus};
use num_traits::{Zero, One};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::mem::replace;
use std::string::String;
use std::vec::Vec;
use std::char;
use std::str;
use std::io;
use std::thread;
use std::fs::File;
use std::io::Write;


// Adapted from the rustlings introduction print :) 
pub fn print_rsa(bit_size: u64){ 
    println!(r#"  _ _ _ _ _ _ _ _ _ __"#);
    println!(r#"°/  _ __  ___  ___    \°"#);
    println!(r#"|  |  __|/ __|/ _ \    |"#);
    println!(r#"|  | |   \__ \ /_\ \   |"#);
    println!(r#"|  |_|   |___//   \_\  |"#);
    println!(r#"+---------[{}]--------+"#, bit_size);
}


// 
pub fn hash(algo: String, msg: String){
    if algo != "md5" {
        println!("The only supported hash is md5")
    }
    println!("Hashing not yet implemented ! Your message is: {}", msg);
}


pub fn cli_keys(bit_size: u64, k_mil_rab: usize, output: String){
    let ((n,e), (d, euler_ind)) = gen_keys(bit_size, k_mil_rab);
    
    println!("\nThe public key in decimal is the pair 
    [ \nn = {}  \n\ne = {} \n]", n, e);
    println!("\nThe private key in decimal is \nd = {}", d);
    
    let zero = 0.to_bigint().unwrap();
    let hex_d  = if d < zero{
        let one = 1.to_bigint().unwrap();
        let _euler_ind = BigInt::from_biguint(Plus, euler_ind);
        let d = d.modpow(&one, &_euler_ind).to_biguint().unwrap();
        naive_hex_from_biguint(&d)
    } else {
        let d = d.to_biguint().unwrap();
        naive_hex_from_biguint(&d)
    };

    let hex_n = naive_hex_from_biguint(&n);
    let hex_e = naive_hex_from_biguint(&e);
    
    println!("\n\n\nThe public key in hexadecimal is the pair 
    [ \nn = 0x{}  \n\ne = 0x{} \n]", hex_n, hex_e);
    println!("\nThe private key in hexdecimal is \nd = 0x{}", hex_d);

    let mut private_key_path = output.clone();
    private_key_path.push_str("/rsa_key");
    let mut file_private = File::create(private_key_path)
                                    .expect("Failed to create private key file");
    file_private.write_all(b"-----BEGIN RSA PRIVATE KEY-----")
                .expect("Failed to write in private key file");
    let len_line = 50;
    for i in 0..hex_d.len()/len_line{
        file_private.write_all(b"\n")
                    .expect("Failed to write in private key file");
        file_private.write_all(hex_d[i*len_line..(i+1)*len_line].as_bytes())
                    .expect("Failed to write in private key file");
    }
    file_private.write_all(b"\n------END RSA PRIVATE KEY------")
                .expect("Failed to write in private key file");

    let mut public_key_path = output;
    public_key_path.push_str("/rsa_key.pub");
    let mut file_public = File::create(public_key_path)
                                .expect("Failed to create private key file");
    file_public.write_all(b"rsa ")
               .expect("Failed to write in private key file");
    let mut long_public = hex_n;
    long_public.push('/');
    long_public.push_str(&hex_e);
    file_public.write_all(long_public.as_bytes())
               .expect("Failed to write in private key file");

}

pub fn run_interactive(bit_size: u64, k_mil_rab: usize, vig_key_size: usize){ 
    let ((n,e), (d, euler_ind)) = gen_keys(bit_size, k_mil_rab);

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
        println!("{}",deciphered_msg);
    
    }
}

fn gen_keys(bit_size: u64, k_mil_rab: usize) -> ((BigUint, BigUint), (BigInt, BigUint)){

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

fn encrypt(msg: &String, vigenere: &Vec<BigUint>,
               e: &BigUint, n: &BigUint) -> (Vec<BigUint>, Vec<BigUint>){
    // transform msg to vec
    let vec_msg : Vec<BigUint> = msg.as_bytes()
                                    .to_vec()
                                    .iter()
                                    .map(|x| x.to_biguint().unwrap())
                                    .collect();
    // encrypt msg with plain Vigenere key
    let enc_vec_msg = vec_msg.iter()
                             .enumerate()
                             .map(|(pos, x)| 
                                (x + &vigenere[ pos % vigenere.len() ]).modpow(e, n))
                             .collect();
    // encrypt Vigenere key with public key (n,e)
    let enc_vigenere = vigenere.iter()
                               .map(|x| x.modpow(e, n))
                               .collect();
    
    (enc_vigenere, enc_vec_msg)
}

fn gen_vigenere(key_size: &usize, n: &BigUint) -> Vec<BigUint>{
    let mut vigenere = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..*key_size{
        // generate a big random key 
        let temp = rng.gen_biguint_below(n);
        // add it to the Vigenere stream 
        vigenere.push(temp);
    }
    vigenere
}

fn decrypt(cipher: (&Vec<BigUint>, &Vec<BigUint>), 
               d: &BigInt, euler_ind: &BigUint, n: &BigUint) -> String {
    let (enc_vigenere, enc_vec_msg) = &cipher;
    //  decrypt the encrypted Vigenere key
    let vigenere: Vec<BigUint> = enc_vigenere.iter()
                                             .map(|x| decipher(x, d, euler_ind, n))
                                             .collect();
    //  decrypt the msg with the plain Vigenere key
    let vec_msg: Vec<u8> = enc_vec_msg.iter()
                                      .enumerate()
                                      .map(|(pos, x)| decipher(x, d, euler_ind, n)
                                                    - &vigenere[ pos % vigenere.len() ])
                                      .map(|x| x.to_string()
                                                .parse::<u8>() 
                                                .unwrap())
                                      .collect();                                    
    str::from_utf8(&vec_msg).unwrap().to_string()

}


fn decipher(cipher: &BigUint, d: &BigInt, 
            euler_ind: &BigUint, n: &BigUint) -> BigUint {

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




// ALGEBRA ON NUMBERS
fn hex_from_uint() -> HashMap<BigUint, char> { 
    HashMap::from([
        (0.to_biguint().unwrap(), '0'), 
        (1.to_biguint().unwrap(), '1'), 
        (2.to_biguint().unwrap(), '2'), 
        (3.to_biguint().unwrap(), '3'),  
        (4.to_biguint().unwrap(), '4'), 
        (5.to_biguint().unwrap(), '5'), 
        (6.to_biguint().unwrap(), '6'), 
        (7.to_biguint().unwrap(), '7'), 
        (8.to_biguint().unwrap(), '8'), 
        (9.to_biguint().unwrap(), '9'), 
        (10.to_biguint().unwrap(), 'a'), 
        (11.to_biguint().unwrap(), 'b'),
        (12.to_biguint().unwrap(), 'c'), 
        (13.to_biguint().unwrap(), 'd'), 
        (14.to_biguint().unwrap(), 'e'), 
        (15.to_biguint().unwrap(), 'f')  
    ])
}

fn naive_hex_from_biguint(num: &BigUint) ->  String{
    // converting a biguint to hexadecimal
    let zero = 0.to_biguint().unwrap();
    let one = 1.to_biguint().unwrap();
    let sixteen = 16.to_biguint().unwrap();
    let mut num2 = num.clone();
    let mut remainder = num2.modpow(&one, &sixteen);
    let mut vec = Vec::new();
    vec.push(remainder.clone());
    while num2 > zero {
        num2 = num2.clone() / sixteen.clone();
        remainder = num2.modpow(&one, &sixteen);
        if num2 > zero{
            vec.push(remainder.clone());
        }
    }
    vec.reverse();
    let mut s = String::new();
    let hashes = hex_from_uint();
    for x in &vec{
        // println!("{x}");
        s.push(hashes[x])
    }
    s
}

fn gen_rand_inverses_below(num: &BigUint) -> (BigUint, BigInt){
    // generating two numbers e,d <= num that are inverses in the ring Z/(num Z)
    let mut rng = rand::thread_rng();
    loop {
        let lower_num = rng.gen_biguint_below(num);
        let res = are_prime_one_another(num, &lower_num);
        if res.1 {
            return (lower_num.clone(), res.0[&lower_num].clone())
        }
    }
}

fn are_prime_one_another(a: &BigUint, b: &BigUint) -> (HashMap<BigUint, BigInt>, bool){
    let one : BigUint = One::one();
    let (coefs, gcd) = euclid_algo(a,b);
    (coefs, gcd == one)
}

fn euclid_algo(a: &BigUint, b: &BigUint) 
        -> (HashMap<BigUint, BigInt>, BigUint) {
    // forming an equation of the form a*u + b*v = gcd(a,b)
    //  where u and v are positive or negative integers
    let zero : BigUint = Zero::zero();
    let one : BigUint = One::one();
    let mut q = min(a.clone(), b.clone());
    let mut p = max(a.clone(), b.clone());
    let temp_p = p.clone();
    let temp_q = q.clone();   
    let mut r = p.modpow(&one, &q); 
    let mut temp_r =  r.clone();
    let mut u0 = BigInt::from_biguint(Plus, zero.clone());
    let mut v0 = BigInt::from_biguint(Plus, one.clone());
    let mut u1 = BigInt::from_biguint(Plus, one.clone()); 
    let mut v1 = BigInt::from_biguint(Minus, p.clone()/q.clone());
    
    while r > zero{
        temp_r = r.clone();
        p = replace(&mut q, r);
        r = BigUint::modpow(&p, &one, &q.clone());
        let s = BigInt::from_biguint(Plus, p.clone() / q.clone());
        let temp_u1 = u1.clone();
        let temp_v1 = v1.clone();
        u0 = replace(&mut u1, &u0 - &s*temp_u1);
        v0 = replace(&mut v1, &v0 - &s*temp_v1);        
    }

    let mut dico = HashMap::new();
    dico.insert(temp_p, u0);
    dico.insert(temp_q.clone(), v0);

    if temp_r != zero {
        (dico, temp_r)
    } else {
        (dico, temp_q)
    }
}


// GENERATION OF PRIME NUMBERS

fn gen_prime(bit_size: u64, k: usize) -> BigUint{
    // generating a prime number greater or equal to 4, way bigger even
    if bit_size <= 2 {
        panic!("Please set bit_size to a number greater or equal to 3");
    }

    loop {
        let num: BigUint = gen_random_odd_biguint(bit_size);
        if is_prime(&num, k){
            return num
        } 
    }
    
}

fn gen_random_odd_biguint(bit_size: u64) -> BigUint{
    // generating a random odd biguint
    let mut rng = rand::thread_rng();
    let mut num = rng.gen_biguint(bit_size);
    // the lowest bit should be 1 to get an odd number
    num.set_bit(0, true);
    // the largest bit equal to 1 to get a number with b_size bits.
    num.set_bit(bit_size-1, true);

    num
}

fn is_prime(b_num: &BigUint, k: usize) -> bool{
    // check whether or not b_num is a prime number
    if !little_fermat(b_num){
        return false
    }

    if !miller_rabin(b_num, k){
        return false
    }
    true
}

fn little_fermat(bnum: &BigUint) -> bool{   
    let mut rng = rand::thread_rng();
    let lower_num = rng.gen_biguint_below(bnum); 
    let one: BigUint = One::one();
    let remainder =  lower_num.modpow(&(bnum - &one), bnum);
    remainder == one
}

fn miller_rabin(num: &BigUint, k: usize) -> bool{
    let one: BigUint = One::one();
    let two = &one + &one;
    for _ in 1..k{
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_biguint_range(&two, &(num - &two)); 
        if is_miller_witness(num, &rand_num){
            return false
        }
    }
    true
}

fn is_miller_witness(num: &BigUint, rand_num: &BigUint) -> bool{
    // check Miller-Rabin condition of non primality
    // if it returns true then num is not a prime number
    let one: BigUint = One::one();
    let two = &one + &one;
    let num_minus_1 = num - &one;
    let (s,d) = factor_two(&num_minus_1);   
    let mut x = BigUint::modpow(rand_num, &d , num);
    if x == one || x == num_minus_1{
        false
    } else {
        for _ in 1..s - 1{
            x = BigUint::modpow(&x, &two , num);
            if x == num_minus_1{
                return false
            }
        }
        true
    }
}

fn factor_two(num: &BigUint) -> (u32, BigUint){
    // find a couple (s-1, d) such that num = 2^(s-1) * d
    let mut s: u32 = 1;
    let zero : BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;
    let mut remainder =  num.modpow(&one, &two.pow(s));
    while remainder == zero {
        s += 1;
        remainder = num.modpow(&one, &two.pow(s));
    }
    (s-1, num/two.pow(s-1))
}