use clap::{ Parser, Subcommand };
use std::fs::{ self, File };
use std::io::Write;
use std::path::{ Path, PathBuf };

mod primes;
mod euclidean;
mod crypto;
mod modulo_arithmetic;

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
    /// Generate a key pair (public and private keys)
    GenerateKeyPair {
        /// Name of the directory where the keys should be generated
        #[arg(long, default_value = ".")]
        key_directory: String,
        /// Name of the key pair to be generated
        #[arg(long, default_value = "default")]
        key_pair_name: String,
        /// Size of the key, 2048 is a good default, for better but slower encryption select 3072 or 4096
        #[arg(long, default_value = "2048")]
        key_size: u16
    },
    Unknown
}

fn save_key_to(key: &crypto::Key, key_path: PathBuf) -> Result<(), anyhow::Error> {
    let mut public_key_file = File::create(key_path)?;
    public_key_file.write_all(&key.as_bytes())?;
    Ok(())
}

fn create_key_path(key_directory: &str, key_pair_name: &str, key_prefix: &str) -> PathBuf {
    let public_key_file_name = format!("{}_{}.elr", key_pair_name, key_prefix);
    Path::new(&key_directory).join(&public_key_file_name)
}

fn main() -> Result<(), anyhow::Error> {
    let cli = CliInterface::parse();

    match cli.command {
        Command::GenerateKeyPair { key_directory, key_pair_name, key_size } => {
            fs::create_dir_all(&key_directory)?;
            let (public_key, private_key) = crypto::generate_keys(key_size);
            let public_key_path = create_key_path(&key_directory, &key_pair_name, "pub");
            save_key_to(&public_key, public_key_path)?;
            let private_key_path = create_key_path(&key_directory, &key_pair_name, "sec");
            save_key_to(&private_key, private_key_path)?;
            println!("Generated a new key pair {}, {}", key_directory, key_pair_name);
            Ok(())
        },
        _ => {
            let (public_key, private_key) = crypto::generate_keys(2048);
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