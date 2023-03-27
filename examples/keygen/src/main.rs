use clap::Parser;
use cryptography::cli_keys;

#[derive(Parser, Debug)]
#[command(name = "keygen")]
#[command(author = "Georges Mbissane SARR <georgesmbissanes@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Generates public and private keys with RSA.", long_about = None)]
struct Cli {
    /// Number of bits of the numbers to generate
    #[arg(short, long)]
    bit_size: u64,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: usize,

    // Path to the folder where to store the keys
    #[arg(short, long)]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    println!("Code launched with the following arguments {:#?}", cli);

    let bit_size: u64 = cli.bit_size;
    let k_mil_rab: usize = cli.k_mil_rab;
    let output: String = cli.output;

    cli_keys(bit_size, k_mil_rab, output);
}
