use std::collections::HashMap;
use num_bigint::{BigUint, ToBigUint, BigInt, ToBigInt};
use num_bigint::Sign::Plus;

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


pub fn uint_from_hex() -> HashMap<char, BigUint> { 
    HashMap::from([
        ('0', 0.to_biguint().unwrap()), 
        ('1', 1.to_biguint().unwrap()), 
        ('2', 2.to_biguint().unwrap()), 
        ('3', 3.to_biguint().unwrap()),  
        ('4', 4.to_biguint().unwrap()), 
        ('5', 5.to_biguint().unwrap()), 
        ('6', 6.to_biguint().unwrap()), 
        ('7', 7.to_biguint().unwrap()), 
        ('8', 8.to_biguint().unwrap()), 
        ('9', 9.to_biguint().unwrap()), 
        ('a', 10.to_biguint().unwrap()), 
        ('b', 11.to_biguint().unwrap()),
        ('c', 12.to_biguint().unwrap()), 
        ('d', 13.to_biguint().unwrap()), 
        ('e', 14.to_biguint().unwrap()), 
        ('f', 15.to_biguint().unwrap())  
    ])
}

pub fn naive_hex_from_biguint(num: &BigUint) ->  String{
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

pub fn convert_bigint_to_biguint_euclid_algo(u: &BigInt, b: &BigUint)
-> BigUint
{
    // considers a relation : a*u + b*v = g (coming from euclidean algo for example),
    // with u < 0, the goal being to force u to be positive. This is doable because, 
    // a*(u + w*b) + (v - w)*b = g, where u will be replaced by u + w*b, w*b > 0 large enough
    // it suffices to take u + w*b equal to the remainder of the euclidean division of u on b
    let zero = 0u32.to_bigint().unwrap();
    let one = 1u32.to_bigint().unwrap();
    if *u < zero{
        let int_b = BigInt::from_biguint(Plus, b.clone());
        u.modpow(&one, &int_b).to_biguint().unwrap()
    } else {
        u.to_biguint().unwrap()
    }
}