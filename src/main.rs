use clap::{ Parser, Subcommand };
use std::fs;
use std::path::Path;
use euler_cryptor::crypto;

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
    /// Use key to encrypt the contents read from the standard input
    Encrypt {
        /// Path to the key to be used
        #[arg(long, default_value = "default")]
        key_path: String,
    },
    /// Use key to decrypt the contents read from the standard input
    Decrypt {
        /// Path to the key to be used
        #[arg(long, default_value = "default")]
        key_path: String,
    }
}

fn main() -> Result<(), anyhow::Error> {
    let cli = CliInterface::parse();

    match cli.command {
        Command::GenerateKeyPair { key_directory, key_pair_name, key_size } => {
            fs::create_dir_all(&key_directory)?;
            let (public_key, private_key) = euler_cryptor::crypto::generate_keys(key_size);
            let public_key_path = euler_cryptor::io::create_key_path(&key_directory, &key_pair_name, "pub");
            euler_cryptor::io::save_key_to(&public_key, public_key_path.as_path())?;
            let private_key_path = euler_cryptor::io::create_key_path(&key_directory, &key_pair_name, "sec");
            euler_cryptor::io::save_key_to(&private_key, private_key_path.as_path())?;
            println!("Generated a new key pair {}, {}", key_directory, key_pair_name);
            Ok(())
        },
        Command::Encrypt { key_path } => {
            let input = euler_cryptor::io::read_from_stdin()?;
            let key = euler_cryptor::io::read_key_from(&Path::new(&key_path))?;
            let encrypted = euler_cryptor::crypto::encrypt_bytes(&input, &key);
            euler_cryptor::io::write_to_stdout(&encrypted)?;
            Ok(())
        },
        Command::Decrypt { key_path } => {
            let input = euler_cryptor::io::read_from_stdin()?;
            let key = euler_cryptor::io::read_key_from(&Path::new(&key_path))?;
            let decrypted = euler_cryptor::crypto::decrypt_bytes(&input, &key);
            euler_cryptor::io::write_to_stdout(&decrypted)?;
            Ok(())
        }
    }
}

//TODO: Add command line interface
// - Generate public and private keys and store them in some format (base-64 encoded) in two separate files - OK
// - Encrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)
// - Decrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)

//TODO: Use the standard pkcs#8 structure for storing the keys

//TODO: Use logging and support the "verbose" option
//TODO: Avoid using "unwrap"
//TODO: Allow to stream the message contents when encrypting and decrypting (this should allow to encrypt and decrypt larger files)

//TODO: Add examples:
// - How the command line tool can be used to sign and verify messages (signing with the private key)
// - How the command line tool can be used to decrypt messages sent to the addressee and encrypted with the public key of the addressee
// - How the library can be used to encrypt and decrypt data
// - Example of using the library and streaming data when encrypting and decrypting
