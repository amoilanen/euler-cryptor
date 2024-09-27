mod primes;
mod euclidean;
mod keys;

fn main() {
    println!("RSA-like encryption algorithm relying on the Euler number theory theorem will be implemented here...");
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
// - Use BigInt arithmetic