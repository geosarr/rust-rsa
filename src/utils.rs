#[cfg(test)]
mod unit_tests;
// use num_bigint::{RandBigInt, BigInt, BigUint, ToBigUint};

// use num_traits::{Zero, One};
// use std::collections::HashMap;
// use std::string::String;
// use std::vec::Vec;
// use std::char;
// use std::str;
// use std::io;
// use std::thread;

mod design;
mod random_generator;
mod conversion;

pub use design::{print_rsa};
pub use random_generator::{*};
pub use conversion::{*};








// GENERATION OF PRIME NUMBERS

