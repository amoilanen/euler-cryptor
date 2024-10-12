use clap::{ Parser, Subcommand };
use std::fs::{ self, File };
use std::io::Write;
use std::path::Path;

mod primes;
mod euclidean;
mod crypto;
mod modulo_arithmetic;

//Testing: cargo run -- generate-key-pair --output-directory ./target/keys --key-pair-name mykeys

/// Cryptographic utility to help encrypt and decrypt data
#[derive(Parser)]
#[command(name = "euler-cryptor")]
#[command(about = "Command line utility to encrypt and decrypt data", long_about = None)]
struct CliInterface {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    GenerateKeyPair {
        #[arg(short, long, default_value = ".")]
        output_directory: String,
        #[arg(short, long, default_value = "default")]
        key_pair_name: String
    },
    Unknown
}

fn main() -> Result<(), anyhow::Error> {
    let cli = CliInterface::parse();

    match cli.command {
        Command::GenerateKeyPair { output_directory, key_pair_name } => {
            println!("Generating a new key pair {}, {}", output_directory, key_pair_name);
            fs::create_dir_all(&output_directory)?;
            let (public_key, private_key) = crypto::generate_keys();

            let public_key_file_name = format!("{}_pub.eulr", key_pair_name);
            let public_key_file_path = Path::new(&output_directory).join(&public_key_file_name);
            let mut public_key_file = File::create(public_key_file_path)?;
            public_key_file.write_all(&public_key.as_bytes())?;

            let private_key_file_name = format!("{}_sec.eulr", key_pair_name);
            let private_key_file_path = Path::new(&output_directory).join(&private_key_file_name);
            let mut private_key_file = File::create(private_key_file_path)?;
            private_key_file.write_all(&private_key.as_bytes())?;

            Ok(())
        },
        _ => {
            let (public_key, private_key) = crypto::generate_keys();
            let text = "The quick brown fox jumps over the lazy dog";
            let encrypted = crypto::encrypt_bytes(&text.as_bytes().to_vec(), &public_key);
            let encrypted_text = String::from_utf8_lossy(&encrypted);
            println!("Encrypted text: '{}'", encrypted_text);
            let decrypted = crypto::decrypt_bytes(&encrypted, &private_key);
            let decrypted_text = String::from_utf8_lossy(&decrypted);
            println!("Decrypted text: '{}'", decrypted_text);
            Ok(())
        }
    }
}

//TODO: Add command line interface
// - Generate public and private keys and store them in some format (base-64 encoded) in two separate files
// - Encrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)
// - Decrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)

//TODO: Add examples:
// - How the command line tool can be used to sign and verify messages (signing with the private key)
// - How the command line tool can be used to decrypt messages sent to the addressee and encrypted with the public key of the addressee

//TODO: Allow to stream the message contents when encrypting and decrypting

//TODO: Use logging and support the "verbose" option
//TODO: Avoid using "unwrap"