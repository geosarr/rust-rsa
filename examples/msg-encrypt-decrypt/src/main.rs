use clap::Parser;
use cryptography::run_interactive;

#[derive(Parser, Debug)]
#[command(name = "msg-enc-dec")]
#[command(author = "Georges Mbissane SARR <georgesmbissanes@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Encrypts and decrypts a message with RSA.", long_about = None)]
struct Cli {
    /// Number of bits of the numbers to generate
    #[arg(short, long)]
    bit_size: u64,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: usize,

    /// Length of the vigenere key
    #[arg(short, long, default_value_t = 10)]
    vig_key_size: usize,
}

fn main() {
    let cli = Cli::parse();
    println!("Code launched with the following arguments {:#?}", cli);

    let bit_size: u64 = cli.bit_size;
    let k_mil_rab: usize = cli.k_mil_rab;
    let vig_key_size: usize = cli.vig_key_size;

    run_interactive(bit_size, k_mil_rab, vig_key_size);
}
