use rand::seq::SliceRandom;
use rand::Rng;

mod primes;
mod euclidean;
mod crypto;

fn encrypt(data: &Vec<u8>, key: &crypto::Key) -> Vec<u8> {
    //TODO: Implement
    // - Append "{length_in_bytes}:" before the text being encrypted
    // - Split the data into blocks of 8 bytes each
    // - Encrypt every block as "M ^ key.exponent modulo key.modulo", do the arithmetic using BigInt to avoid overflow
    Vec::new()
}

fn decrypt(data: &Vec<u8>, key: &crypto::Key) -> Vec<u8> {
    //TODO: Implement
    Vec::new()
}

fn main() {
    println!("RSA-like encryption algorithm relying on the Euler number theory theorem will be implemented here...");
    //TODO: This is just an initial version, to be secure the primes would have to be selected randomly from a range  > Math.pow(2, 64)
    let primes_bottom: usize = 16777216; //2^24 - allows to encode blocks of 24 / 8 = 3 bytes since a < p, q where n = p * q
    let primes_top: usize = 4294967296; //2^32
    let mut rng = rand::thread_rng();
    let mut primes_from = rng.gen_range(primes_bottom + 1..primes_top);
    let segment_size = 1000;
    let &p: &usize = primes::primes_segment(primes_from, primes_from + segment_size).choose(&mut rng).unwrap();
    let mut q: usize = p;
    while q == p {
        primes_from = rng.gen_range(primes_bottom + 1..primes_top);
        let &new_q: &usize = primes::primes_segment(primes_from, primes_from + segment_size).choose(&mut rng).unwrap();
        q = new_q;
    }
    let n = p * q;
    let totient_function = (p - 1) * (q - 1);
    println!("p={:?}, q={:?}, n={:?}, totient_function={:?}", p, q, n, totient_function);

    let public_exponent: u64 = 65537;

    let gcd_and_bezout_coefficients = euclidean::find_gcd_and_bezout_coefficients(public_exponent as i64, totient_function as i64);
    assert!(gcd_and_bezout_coefficients.gcd == 1);
    let private_exponent: u64 = gcd_and_bezout_coefficients.x as u64;

    let public_key = crypto::Key {
        exponent: public_exponent,
        modulo: n as u64
    };
    let private_key = crypto::Key {
        exponent: private_exponent,
        modulo: n as u64
    };

    let text = "The quick brown fox jumps over the lazy dog";
    let encrypted = encrypt(&text.as_bytes().to_vec(), &public_key);
    let decrypted = decrypt(&encrypted, &private_key);
    let decrypted_text = String::from_utf8_lossy(&decrypted);
    println!("Decrypted text: '{}'", decrypted_text)
}

//TODO: Find two large prime numbers p and q. n = p * q, phi(n) = (p - 1)(q - 1)
//TODO: Choose a large enough public exponent e such as e > 10000, e is prime and e < phi(n)
//TODO: Compute the public and the private exponent (components of the public and private keys), use the extended Euclidean algorithm to solve d * e = 1 (mod phi(n))
//TODO: Implement the encryption and decryption procedures

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

//TODO: Use a padding scheme when ecrypting the message to increase security