#[cfg(test)]
mod tests{
    // use super::{*};
    use crate::utils::*;
    use num_bigint::{RandBigInt, BigInt, BigUint, ToBigUint};
    use num_traits::{Zero, One};
    use num_bigint::Sign::Plus;

    fn set_up() -> (BigUint, BigUint, BigUint){
        let zero : BigUint = Zero::zero();
        let one: BigUint = One::one();
        let two = &one + &one;
        
        (zero, one, two)
    }
    

    #[test]
    fn test_gen_random_odd_biguint(){
        for _ in 0..100000{
            let (zero, one, two) = set_up();
            let bit_size: u64 = 256;
            let pow_two = two.pow(bit_size as u32);
            let pow_min_one_two = two.pow((bit_size-1) as u32);
            let num = gen_random_odd_biguint(bit_size);
            let remainder = num.modpow(&one, &two);
            assert_eq!(true, num.bit(0));
            assert_eq!(true, num.bit(bit_size-1));
            assert_eq!(bit_size, num.bits());
            assert_eq!(zero, num.clone()/pow_two); // at most (bit_size) bits
            assert_eq!(one, num.clone()/pow_min_one_two); // largest bit is 1
            assert_eq!(one, remainder); // lowest bit is set to 1
        }
    }

    #[test]
    fn test_factor_two(){
        for _ in 0..100000{
            let (zero, one, two) = set_up();
            let mut rng = rand::thread_rng();
            let num = rng.gen_biguint(256);
            let (s,d) = factor_two(&num);
            assert_eq!(zero, num.modpow(&one, &two.pow(s)));
            assert_ne!(zero, num.modpow(&one, &two.pow(s+1)));
            assert_eq!(num, two.pow(s) * d);
        }

    }

    #[test]
    fn test_small_num_primality(){
        for _ in 0..10{
            let (zero, one, two) = set_up();
            let bit_size: u64 = 40;
            let test_candidate = gen_prime(bit_size, 3 as usize);
            let sqrt_test_candidate = test_candidate.sqrt();
            let mut odd = &two + &one;
            let remainder = test_candidate.modpow(&one, &odd);
            while odd <= sqrt_test_candidate.clone(){
                assert_ne!(zero, remainder);
                odd = odd.clone() + two.clone();
            }
        }
    }

    #[test]
    fn test_euclid_algo(){
        let mut rng = rand::thread_rng();
        let bit_size: u64 = 40;
        for _ in 0..10000{
            let num1 = rng.gen_biguint(bit_size);
            let num2 = rng.gen_biguint(bit_size);
            let (coefs, gcd) = euclid_algo(&num1, &num2);            
            let int_num1 = BigInt::from_biguint(Plus, num1.clone());
            let int_num2 = BigInt::from_biguint(Plus, num2.clone());
            if num1 != num2{
                assert_eq!(BigInt::from_biguint(Plus, gcd), 
                           coefs[&num1].clone()*int_num1.clone() + coefs[&num2].clone()*int_num2.clone()
                );
            } else{
                assert_eq!(gcd, num1);
                assert_eq!(gcd, num2);
            }
        }
    }

    #[test]
    fn test_gen_rand_inverses_below(){
        let mut rng = rand::thread_rng();
        let bit_size: u64 = 40;
        let (_, one, __) = set_up();
        let int_one = BigInt::from_biguint(Plus, one.clone());
        for _ in 0..1000{
            let num = rng.gen_biguint(bit_size);
            let int_num = BigInt::from_biguint(Plus, num.clone());
            let (e, d) = gen_rand_inverses_below(&num);
            let e_d = d*BigInt::from_biguint(Plus, e);
            assert_eq!(int_one, e_d.modpow(&int_one, &int_num));
        }
    }

    #[test]
    fn test_naive_hex_from_biguint(){
        let mut rng = rand::thread_rng();
        let bit_size: u64 = 40;
        let sixteen = 16.to_biguint().unwrap();
        let ufh = uint_from_hex();
        for _ in 1..10000{
            let num = rng.gen_biguint(bit_size);
            let s = naive_hex_from_biguint(&num);
            let mut c : BigUint = Zero::zero();
            let hex_size : usize = s.len();
            for i in 0..s.len(){
                let temp = s.chars().nth(i).unwrap();
                c += ufh[&temp].clone() * sixteen.pow((hex_size - i - 1) as u32);
            }
            assert_eq!(c, num);
        }
    }

    // #[test]
    // fn test_encrypt_decrypt(){
    //     use std::thread;

    //     let mut rng = rand::thread_rng(); 
    //     let v = vec!['a', '@', ' ', '_', '0', 'é', '$', 'µ'];
    //     let bit_size = 256;
    //     let one = 1.to_biguint().unwrap();
    //     let max_key_size = 50.to_biguint().unwrap();
    //     for _ in 0..10{
    //         // Generate a random string
    //         let mut msg = String::new();
    //         for _ in 0..20{
    //             let max_idx = (v.len()-1).to_biguint().unwrap();
    //             let char_idx = rng.gen_biguint_below(&max_idx);
    //             msg.push(v[char_idx.to_string().parse::<usize>().unwrap()])
    //         }
    //         let vig_key_size = rng.gen_biguint_range(&one, &max_key_size)
    //                               .to_string()
    //                               .parse::<usize>().unwrap();
    //         // Generate keys
    //         let handle = thread::spawn(move || {
    //             (gen_prime(bit_size, 3 as usize), gen_prime(bit_size, 3 as usize))
    //             }
    //         );
    //         let (p,q) = handle.join().unwrap();
    //         let n = &p*&q;    
    //         let euler_ind = (&p-&one) * (&q-&one);
    //         let (e, d) = gen_rand_inverses_below(&euler_ind);
    //         let vigenere = gen_vigenere(&vig_key_size, &n);    
        
    //         let (enc_vigenere, enc_msg) = encrypt(&msg, &vigenere, &e, &n);

    //         let dec_msg = decrypt((&enc_vigenere, &enc_msg), &d, &euler_ind, &n);
    
    //         assert_eq!(msg, dec_msg);

    //     }
    // }
}