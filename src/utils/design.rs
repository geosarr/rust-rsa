// Adapted from the rustlings introduction print :)
pub fn print_rsa(bit_size: u64) {
    println!(r#"  _ _ _ _ _ _ _ _ _ __"#);
    println!(r#"°/  _ __  ___  ___    \°"#);
    println!(r#"|  |  __|/ __|/ _ \    |"#);
    println!(r#"|  | |   \__ \ /_\ \   |"#);
    println!(r#"|  |_|   |___//   \_\  |"#);
    println!(r#"+---------[{}]--------+"#, bit_size);
}
