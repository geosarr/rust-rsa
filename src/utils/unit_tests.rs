
#[cfg(test)]
mod tests{
    use super::super::{*};

    fn set_up() -> (BigUint, BigUint, BigUint){
        let zero : BigUint = Zero::zero();
        let one: BigUint = One::one();
        let two = &one + &one;
        
        (zero, one, two)
    }
    

    #[test]
    fn test_gen_random_odd_biguint(){
        for _ in 1..100000{
            let (zero, one, two) = set_up();
            let bit_size: u64 = 256;
            let pow_two = BigUint::pow(&two, bit_size as u32);
            let pow_min_one_two = BigUint::pow(&two, (bit_size-1) as u32);
            let num = gen_random_odd_biguint(bit_size);
            let remainder = BigUint::modpow(&num, &one, &two);
            assert_eq!(true, num.bit(0));
            assert_eq!(true, num.bit(bit_size-1));
            assert_eq!(zero, num.clone()/pow_two); // at most (bit_size) bits
            assert_eq!(one, num.clone()/pow_min_one_two); // largest bit is 1
            assert_eq!(one, remainder); // lowest bit is set to 1
        }
    }

    #[test]
    fn test_factor_two(){
        for _ in 1..100000{
            let (zero, one, two) = set_up();
            let mut rng = rand::thread_rng();
            let num = rng.gen_biguint(256);
            let (s,d) = factor_two(num.clone());
            assert_eq!(zero, BigUint::modpow(&num, &one, &BigUint::pow(&two, s)));
            assert_ne!(zero, BigUint::modpow(&num, &one, &BigUint::pow(&two, s+1)));
            assert_eq!(num.clone(), BigUint::pow(&two, s) * d);
        }

    }

    #[test]
    fn test_small_num_primality(){
        for _ in 1..10{
            let (zero, one, two) = set_up();
            let bit_size: u64 = 40;
            let test_candidate = gen_prime(bit_size, 3 as u32);
            let sqrt_test_candidate = BigUint::sqrt(&test_candidate);
            let mut odd = &two + &one;
            let remainder = BigUint::modpow(&test_candidate, &one, &odd);
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
        for _ in 1..10000{
            let num1 = rng.gen_biguint(bit_size);
            let num2 = rng.gen_biguint(bit_size);
            let (coefs, gcd) = euclid_algo(num1.clone(), num2.clone());            
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
        for _ in 1..1000{
            let num = rng.gen_biguint(bit_size);
            let int_num = BigInt::from_biguint(Plus, num.clone());
            let (e, d) = gen_rand_inverses_below(num);
            let e_d = d*BigInt::from_biguint(Plus, e);
            assert_eq!(int_one, BigInt::modpow(&e_d, &int_one, &int_num));
        }
    }
}