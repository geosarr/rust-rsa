use num_bigint::{RandBigInt, BigInt, BigUint, ToBigUint};
use num_traits::{Zero, One};
use std::collections::HashMap;
use std::cmp::{min, max};
use num_bigint::{ToBigInt, Sign::Plus, Sign::Minus};
use std::mem::replace;


pub fn gen_vigenere(key_size: &usize, n: &BigUint) -> Vec<BigUint>{
    // generates a Vigenere stream of size key_size from BigUint <= n
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

pub fn gen_rand_inverses_below(num: &BigUint) -> (BigUint, BigInt){
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
    // tests whether or not a and b do not have a common prime divisor
    // and returns at the same time coefficients satisfying the "generalized" Bezout formula:
    // coefs[a]*a + coefs[b]*b = gcd(a,b).
    let one : BigUint = One::one();
    let (coefs, gcd) = euclid_algo(a,b);
    (coefs, gcd == one)
}

pub fn euclid_algo(a: &BigUint, b: &BigUint) 
        -> (HashMap<BigUint, BigInt>, BigUint) {
    // forming an equation of the form a*u + b*v = gcd(a,b)
    // where u and v are positive or negative integers
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



// PRIME NUMBER GENERATION
pub fn gen_prime(bit_size: u64, k: usize) -> BigUint{
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

pub fn gen_random_odd_biguint(bit_size: u64) -> BigUint{
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

pub fn factor_two(num: &BigUint) -> (u32, BigUint){
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