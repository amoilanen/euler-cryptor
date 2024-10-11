mod primes;
mod euclidean;
mod crypto;
mod modulo_arithmetic;

fn main() {
    let (public_key, private_key) = crypto::generate_keys();
    let text = "The quick brown fox jumps over the lazy dog";
    let encrypted = crypto::encrypt_bytes(&text.as_bytes().to_vec(), &public_key);
    let encrypted_text = String::from_utf8_lossy(&encrypted);
    println!("Encrypted text: '{}'", encrypted_text);
    let decrypted = crypto::decrypt_bytes(&encrypted, &private_key);
    let decrypted_text = String::from_utf8_lossy(&decrypted);
    println!("Decrypted text: '{}'", decrypted_text)
}

//TODO: Add command line interface
// - Generate public and private keys and store them in some format (base-64 encoded) in two separate files
// - Encrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)
// - Decrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)

//TODO: Add examples:
// - How the command line tool can be used to sign and verify messages (signing with the private key)
// - How the command line tool can be used to decrypt messages sent to the addressee and encrypted with the public key of the addressee

//TODO: Allow to stream the message contents when encrypting and decrypting

//TODO: Avoid using "unwrap"