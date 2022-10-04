// mod utils; 
// use num_bigint::BigUint;
// use num_bigint::{ToBigInt, RandBigInt, BigUint};
use num_bigint::{RandBigInt, BigUint};
use num_traits::One;

fn main(){

    while true {
        let bit_size: u64 = 256; 
        let num: BigUint = gen_random_odd_biguint(bit_size);
        // cloning to prevent errors due to data move and borrowing
        if !is_prime(num.clone()){
            println!("not prime");
            let num = gen_random_odd_biguint(bit_size);
            println!("{}", num);
        } else{
            println!("YUPI-HAPPY {} is probably a prime number", num);
            break;
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

fn is_prime(b_num: BigUint) -> bool{
    if !little_fermat(b_num){
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