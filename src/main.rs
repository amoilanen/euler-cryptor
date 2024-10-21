use clap::{ Parser, Subcommand };
use std::fs::{self, read};
use std::io::Read;
use std::path::Path;

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
            let mut input = euler_cryptor::io::stdin_stream()?;
            let key = euler_cryptor::io::read_key_from(&Path::new(&key_path))?;
            let chunk_size = euler_cryptor::crypto::encryption_block_size(&key);
            let mut buffer = vec![0u8; chunk_size];
            let mut read_bytes_size = 1;
            while read_bytes_size != 0 {
                read_bytes_size = input.read(&mut buffer)?;
                let read_bytes = buffer[0..read_bytes_size].to_vec();
                /*
                if read_bytes_size != chunk_size {
                    println!("Expected to encrypt a chunk of {} bytes, but read {} bytes", chunk_size, read_bytes_size);
                }
                */
                let encrypted = euler_cryptor::crypto::encrypt_bytes(&read_bytes, &key);
                euler_cryptor::io::write_to_stdout(&encrypted)?;
            }
            Ok(())
        },
        Command::Decrypt { key_path } => {
            let mut input = euler_cryptor::io::stdin_stream()?;
            let key = euler_cryptor::io::read_key_from(&Path::new(&key_path))?;
            let chunk_size = euler_cryptor::crypto::decryption_block_size(&key);
            println!("chunk_size = {}", chunk_size);
            let mut buffer = vec![0u8; chunk_size];
            let mut read_buffer_size = 0;
            let mut read_bytes_size = 1;
            while read_bytes_size != 0 {
                read_bytes_size = input.read(&mut buffer[read_buffer_size..])?;
                read_buffer_size = read_buffer_size + read_bytes_size;
                if read_bytes_size == 0 {
                    if read_buffer_size > 0 {
                        let read_bytes = buffer[0..read_buffer_size].to_vec();
                        read_buffer_size = 0;
                        let decrypted = euler_cryptor::crypto::decrypt_bytes(&read_bytes, &key);
                        euler_cryptor::io::write_to_stdout(&decrypted)?;
                    }
                } else if read_buffer_size == chunk_size {
                    let read_bytes = buffer[0..read_buffer_size].to_vec();
                    read_buffer_size = 0;
                    let decrypted = euler_cryptor::crypto::decrypt_bytes(&read_bytes, &key);
                    euler_cryptor::io::write_to_stdout(&decrypted)?;
                }
            }
            Ok(())
        }
    }
}

//TODO: Optimize encryption and decryption of larger files
//TODO: Allow to stream the message contents when encrypting and decrypting (this should allow to encrypt and decrypt larger files)
//TODO: Use the standard pkcs#8 structure for storing the keys

//TODO: Use logging and support the "verbose" option
//TODO: Avoid using "unwrap"

//TODO: Add examples:
// - How the command line tool can be used to sign and verify messages (signing with the private key)
// - How the command line tool can be used to decrypt messages sent to the addressee and encrypted with the public key of the addressee
// - How the library can be used to encrypt and decrypt data
// - Example of using the library and streaming data when encrypting and decrypting
