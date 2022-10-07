#[cfg(test)]
mod unit_tests;
use num_bigint::{RandBigInt, BigInt, BigUint, ToBigUint, Sign::Plus, Sign::Minus};
use num_traits::{Zero, One};
use std::collections::HashMap;
use std::cmp::{min, max};
use std::mem::replace;
use std::string::String;
use std::vec::Vec;
use std::char;

// SOME CONSTANTS
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
// Adapted from the rustlings introduction print :) 
pub fn print_rsa(bit_size: u64){ 
    println!(r#"  _ _ _ _ _ _ _ _ _ __"#);
    println!(r#"°/  _ __  ___  ___    \°"#);
    println!(r#"|  |  __|/ __|/ _ \    |"#);
    println!(r#"|  | |   \__ \ /_\ \   |"#);
    println!(r#"|  |_|   |___//   \_\  |"#);
    println!(r#"+---------[{}]---------+"#, bit_size);
}

// ALGEBRA ON NUMBERS

pub fn naive_hex_from_biguint(num: BigUint) ->  String{
    let zero = 0.to_biguint().unwrap();
    let one = 1.to_biguint().unwrap();
    let sixteen = 16.to_biguint().unwrap();
    let mut num2 = num.clone();
    let mut remainder = num2.modpow(&one, &sixteen);
    let mut vec = Vec::new();
    vec.push(remainder.clone());
    while num2.clone() > zero {
        num2 = num2 / sixteen.clone();
        remainder = num2.modpow(&one, &sixteen);
        if num2.clone() > zero{
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

pub fn gen_rand_inverses_below(num: BigUint) -> (BigUint, BigInt){
    let mut rng = rand::thread_rng();
    loop {
        let lower_num = rng.gen_biguint_below(&num);
        let res = are_prime_one_another(num.clone(), lower_num.clone());
        if res.1 {
            return (lower_num.clone(), res.0[&lower_num].clone())
        }
    }
}

fn are_prime_one_another(a: BigUint, b: BigUint) -> (HashMap<BigUint, BigInt>, bool){
    let one : BigUint = One::one();
    let (coefs, gcd) = euclid_algo(a,b);
    (coefs, gcd == one)
}

pub fn euclid_algo(a: BigUint, b: BigUint) 
        -> (HashMap<BigUint, BigInt>, BigUint) {
    let zero : BigUint = Zero::zero();
    let one : BigUint = One::one();
    let mut q = min(a.clone(), b.clone());
    let mut p = max(a.clone(), b.clone());
    let temp_p = p.clone();
    let temp_q = q.clone();   
    let mut r = BigUint::modpow(&p, &one, &q.clone()); 
    let mut temp_r =  r.clone();
    let mut u0 = BigInt::from_biguint(Plus, zero.clone());
    let mut v0 = BigInt::from_biguint(Plus, one.clone());
    let mut u1 = BigInt::from_biguint(Plus, one.clone()); 
    let mut v1 = BigInt::from_biguint(Minus, p.clone() /q.clone());
    let mut dico = HashMap::new();
    while r.clone() > zero.clone(){
        temp_r = r.clone();
        p = replace(&mut q, r);
        r = BigUint::modpow(&p, &one, &q.clone());
        let s = BigInt::from_biguint(Plus, p.clone() / q.clone());
        let temp_u1 = u1.clone() ;
        let temp_v1 = v1.clone() ;
        u1 = u0 - &s * u1.clone() ; 
        v1 = v0 - &s * v1.clone() ;
        u0 = temp_u1; 
        v0 = temp_v1; 
    }
    dico.insert(temp_p.clone(), u0);
    dico.insert(temp_q.clone(), v0);
    if temp_r.clone() != zero.clone() {
        (dico, temp_r.clone())
    } else {
        (dico, temp_q.clone())
    }
}


// GENERATION OF PRIME NUMBERS

pub fn gen_prime(bit_size: u64, k: u32) -> BigUint{
    if bit_size <= 2{
        panic!("Please set bit_size to a number greater or equal to 3");
    }

    loop {
        
        let num: BigUint = gen_random_odd_biguint(bit_size);
        if is_prime(num.clone(), k){
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

fn is_prime(b_num: BigUint, k: u32) -> bool{
    if !little_fermat(b_num.clone()){
        return false
    }

    if !miller_rabin(b_num.clone(), k){
        return false
    }
    true
}

fn little_fermat(bnum: BigUint) -> bool{   
    let mut rng = rand::thread_rng();
    let lower_num = rng.gen_biguint_below(&bnum); 
    let one: BigUint = One::one();
    let remainder =  BigUint::modpow(&lower_num, &(&bnum - &one), &bnum);
    remainder == one
}

fn miller_rabin(num: BigUint, k: u32) -> bool{
    let one: BigUint = One::one();
    let two = &one + &one;
    for _ in 1..k{
        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_biguint_range(&two, &(&num - &two)); 
        if is_miller_witness(num.clone(), rand_num){
            return false
        }
    }
    true
}

fn is_miller_witness(num: BigUint, rand_num: BigUint) -> bool{
    let one: BigUint = One::one();
    let two = &one + &one;
    let (s,d) = factor_two(num.clone() - one.clone());   
    let mut x = BigUint::modpow(&rand_num, &d ,&num);
    if x == one || x == num.clone() - one.clone(){
        return false
    } else {
        for _ in 1..s - 1{
            x = BigUint::modpow(&x, &two , &num);
            if x == num.clone() - one.clone(){
                return false
            }
        }
        true
    }
}

fn factor_two(num: BigUint) -> (u32, BigUint){
    let mut s: u32 = 1;
    let zero : BigUint = Zero::zero();
    let one: BigUint = One::one();
    let two = &one + &one;
    let mut remainder = BigUint::modpow(&num, &one, &BigUint::pow(&two, s));
    while remainder == zero {
        s = s + 1;
        remainder = BigUint::modpow(&num, &one, &BigUint::pow(&two, s));
    }
    (s-1, num.clone()/BigUint::pow(&two, s-1))
}
