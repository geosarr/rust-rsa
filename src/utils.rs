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

// SOME "CONSTANTS"

// Adapted from the rustlings introduction print :) 
pub fn print_rsa(bit_size: u64){ 
    println!(r#"  _ _ _ _ _ _ _ _ _ __"#);
    println!(r#"°/  _ __  ___  ___    \°"#);
    println!(r#"|  |  __|/ __|/ _ \    |"#);
    println!(r#"|  | |   \__ \ /_\ \   |"#);
    println!(r#"|  |_|   |___//   \_\  |"#);
    println!(r#"+---------[{}]--------+"#, bit_size);
}

pub fn hex_from_uint() -> HashMap<BigUint, char> { 
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


pub fn encrypt(msg: &String, vigenere: &Vec<BigUint>,
               e: &BigUint, n: &BigUint) -> (Vec<BigUint>, Vec<BigUint>){
    let vec_msg : Vec<BigUint> = msg.as_bytes()
                                    .to_vec()
                                    .iter()
                                    .map(|x| x.to_biguint().unwrap())
                                    .collect();
    let enc_vec_msg = vec_msg.iter()
                             .enumerate()
                             .map(|(pos, x)| 
                                (x + &vigenere[ pos % &vigenere.len() ]).modpow(e, n))
                             .collect();
    let enc_vigenere = vigenere.iter()
                               .map(|x| x.modpow(e, n))
                               .collect();
    
    (enc_vigenere, enc_vec_msg)
}

pub fn gen_vigenere(key_size: &usize, n: &BigUint) -> Vec<BigUint>{
    let mut vigenere = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..key_size.clone(){
        let temp = rng.gen_biguint_below(n); 
        vigenere.push(temp);
    }
    vigenere
}

pub fn decrypt(cipher: (&Vec<BigUint>, &Vec<BigUint>), 
               d: &BigInt, euler_ind: &BigUint, n: &BigUint) -> String {
    let (enc_vigenere, enc_vec_msg) = &cipher;
    let vigenere: Vec<BigUint> = enc_vigenere.iter()
                                             .map(|x| decipher(&x, d, euler_ind, n))
                                             .collect();
    let vec_msg: Vec<u8> = enc_vec_msg.iter()
                                      .enumerate()
                                      .map(|(pos, x)| decipher(&x, d, euler_ind, n)
                                                    - &vigenere[ pos % &vigenere.len() ])
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
    // Conditions due to impossibilty (now) to compute modpow with negative power 
    if d >= &zero {
        let _d = d.to_biguint().unwrap();
        cipher.modpow(&_d, &n)
    } else {
        let _euler_ind = BigInt::from_biguint(Plus, euler_ind.clone());
        let remainder = d.modpow(&one, &_euler_ind).to_biguint().unwrap();
        cipher.modpow(&remainder, &n)
    }

}




// ALGEBRA ON NUMBERS

pub fn naive_hex_from_biguint(num: &BigUint) ->  String{
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
        s.push(hashes[&x])
    }
    s
}

pub fn gen_rand_inverses_below(num: &BigUint) -> (BigUint, BigInt){
    let mut rng = rand::thread_rng();
    loop {
        let lower_num = rng.gen_biguint_below(&num);
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

pub fn euclid_algo(a: &BigUint, b: &BigUint) 
        -> (HashMap<BigUint, BigInt>, BigUint) {
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
    
    while &r > &zero{
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
    dico.insert(temp_p.clone(), u0);
    dico.insert(temp_q.clone(), v0);

    if temp_r.clone() != zero.clone() {
        (dico, temp_r.clone())
    } else {
        (dico, temp_q.clone())
    }
}


// GENERATION OF PRIME NUMBERS

pub fn gen_prime(bit_size: u64, k: usize) -> BigUint{
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

pub fn gen_random_odd_biguint(bit_size: u64) -> BigUint{
    let mut rng = rand::thread_rng();
    let mut num = rng.gen_biguint(bit_size);
    // the lowest bit should be 1 to get an odd number
    num.set_bit(0, true);
    // the largest bit equal to 1 to get a number with b_size bits.
    num.set_bit(bit_size-1, true);

    num
}

fn is_prime(b_num: &BigUint, k: usize) -> bool{
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
    let one: BigUint = One::one();
    let two = &one + &one;
    let num_minus_1 = num - &one;
    let (s,d) = factor_two(&num_minus_1);   
    let mut x = BigUint::modpow(rand_num, &d , num);
    if x == one || x == num_minus_1{
        return false
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
    let mut s: u32 = 1;
    let zero : BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;
    let mut remainder =  num.modpow(&one, &two.pow(s));
    while remainder == zero {
        s = s + 1;
        remainder = num.modpow(&one, &two.pow(s));
    }
    (s-1, num/two.pow(s-1))
}
