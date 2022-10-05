mod utils;
use utils::{gen_prime};

fn main(){
    let bit_size: u64 = 500; 
    let k_mil_rab: u32 = 3;
    gen_prime(bit_size, k_mil_rab);
}