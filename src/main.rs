mod primes;
mod euclidean;
mod crypto;

fn main() {
    println!("RSA-like encryption algorithm relying on the Euler number theory theorem will be implemented here...");
    let (public_key, private_key) = crypto::generate_keys();
    let text = "The quick brown fox jumps over the lazy dog";
    let encrypted = crypto::encrypt_bytes(&text.as_bytes().to_vec(), &public_key);
    let encrypted_text = String::from_utf8_lossy(&encrypted);
    println!("Encrypted text: '{}'", encrypted_text);
    let decrypted = crypto::decrypt_bytes(&encrypted, &private_key);
    let decrypted_text = String::from_utf8_lossy(&decrypted);
    println!("Decrypted text: '{}'", decrypted_text)
}

//TODO: Avoid using "unwrap"

//TODO: Add command line interface
// - Generate public and private keys and store them in some format (base-64 encoded) in two separate files
// - Encrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)
// - Decrypt Vec<u8> input using the provided key (can be either public or private due to the symmetric nature of the algorithm)

//TODO: Add examples:
// - How the command line tool can be used to sign and verify messages (signing with the private key)
// - How the command line tool can be used to decrypt messages sent to the addressee and encrypted with the public key of the addressee

//TODO: Use very large prime numbers, i.e. hundreds of digits.
// - Use AKS primality test starting with a random  number  and crossing out all the multiples of the prime numbers less than 10000(optimization)
// - Use BigInt arithmetic and avoid the limitations of u64 (and avoid u64 <-> i64 conversions especially and ignoring potential overflows)

//TODO: Allow to stream the message contents when encrypting and decrypting