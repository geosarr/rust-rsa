
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

}