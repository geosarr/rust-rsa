#[cfg(test)]
mod unit_tests;
use num_bigint::{RandBigInt, BigUint};
use num_traits::{Zero, One};

pub fn gen_prime(bit_size: u64, k: u32) -> BigUint{
    if bit_size <= 2{
        panic!("Please set bit_size to a number greater or equal to 3");
    }

    loop {
        
        let num: BigUint = gen_random_odd_biguint(bit_size);
        // cloning to prevent errors due to data move and borrowing
        if is_prime(num.clone(), k){
            println!("YUPI-HAPPY {} is probably a prime number", num);
            return num
        } 
    }
    
}

fn gen_random_odd_biguint(bit_size: u64) -> BigUint{
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
    // let two: u32 = 2;
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
