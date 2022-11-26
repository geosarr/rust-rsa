mod utils;
use utils::{run_interactive, cli_keys, hash};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "RSA")]
#[command(author = "Georges Mbissane SARR <georgesmbissanes@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Generates public and private keys with RSA.", long_about = None)]
struct Cli {
    /// Number of bits of the numbers to generate
    #[arg(short, long, default_value_t = 512)]
    bit_size: u64,

    /// Interactive mode
    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    /// Number of times to run Miller-Rabin test
    #[arg(short, long, default_value_t = 3)]
    k_mil_rab: usize,

    /// Length of the vigenere key
    #[arg(short, long, default_value_t = 10)]
    vig_key_size: usize, 

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a public/private pair of keys  
    Keygen { 
        /// Number of bits of the numbers to generate
        #[arg(short, long)]
        bit_size: u64, 

        /// Number of times to run Miller-Rabin test
        #[arg(short, long, default_value_t = 3)]
        k_mil_rab: usize,

        // Path to the folder where to store the keys
        #[arg(short, long,)]
        output: String,
    },

    Hash {
        /// Hashing algorithm for a message
        #[arg(short)]
        algo: String,

        /// Message to hash
        #[arg(short)]
        msg: String,
    }
}

fn main(){
    let cli = Cli::parse();
    // println!("");
    // println!("Code launched with the following arguments {:#?}", cli);

    let bit_size: u64 = cli.bit_size;
    let k_mil_rab: usize = cli.k_mil_rab;
    let vig_key_size: usize = cli.vig_key_size;

    if cli.interactive {
        run_interactive(bit_size, k_mil_rab, vig_key_size);
    }

    match &cli.command {
        Some(Commands::Keygen{bit_size, k_mil_rab, output}) => cli_keys(*bit_size, *k_mil_rab, output.clone()),
        Some(Commands::Hash{algo, msg}) => hash(algo.clone(), msg.clone()),
        None => println!("Use the flag --help to see how to run the rsa program"),
    };

}

