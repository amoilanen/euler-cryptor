use num_bigint::{BigInt, Sign};
use num_traits::One;
use num_traits::{FromPrimitive, Zero};
use std::cmp;
use rand::Rng;

use crate::euclidean;
use crate::primes;
use crate::modulo_arithmetic;

#[derive(Debug, PartialEq)]
pub struct Key {
    pub exponent: BigInt,
    pub modulo: BigInt
}

impl Key {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let exponent_bytes = self.exponent.to_bytes_be();
        assert_eq!(exponent_bytes.0, Sign::Plus);
        let modulo_bytes = self.modulo.to_bytes_be();
        assert_eq!(modulo_bytes.0, Sign::Plus);
        result.extend((exponent_bytes.1.len() as u32).to_be_bytes());
        result.extend((modulo_bytes.1.len() as u32).to_be_bytes());
        result.extend(exponent_bytes.1);
        result.extend(modulo_bytes.1);
        result
    }

    pub fn from_bytes(bytes: &Vec<u8>) -> Result<Key, anyhow::Error> {
        let exponent_length = u32::from_be_bytes(bytes[0..4].try_into()?) as usize;
        let modulo_length = u32::from_be_bytes(bytes[4..8].try_into()?) as usize;
        let exponent_bytes: Vec<u8> = bytes[8..(8 + exponent_length)].to_vec();
        let modulo_bytes: Vec<u8> = bytes[(8 + exponent_length)..(8 + exponent_length + modulo_length)].to_vec();
        Ok(Key {
            exponent: BigInt::from_bytes_be(Sign::Plus, &exponent_bytes),
            modulo: BigInt::from_bytes_be(Sign::Plus, &modulo_bytes)
        })
    }
}

fn find_private_key(totient_function: &BigInt, public_key: &BigInt) -> BigInt {
    let gcd_and_coefficients = euclidean::find_gcd_and_bezout_coefficients(public_key, totient_function);
    let mut private_key = gcd_and_coefficients.y;
    if private_key < BigInt::from_u8(0).unwrap() {
        private_key = private_key + totient_function;
    }
    private_key
}

const PUBLIC_EXPONENT: u32 = 65537;

fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut result = vec![0u8; size];
    let mut rng = rand::thread_rng();
    rng.fill(&mut result[..]);
    result
}

fn is_prime(n: &BigInt, first_primes: &Vec<usize>) -> bool {
    if first_primes.iter().any(|prime| n % prime == BigInt::zero()) {
        false
    } else {
        primes::miller_rabin_primality_test(n)
    }
}

fn find_random_prime(prime_bits: usize, first_primes: &Vec<usize>) -> BigInt {
    let prime_bytes = prime_bits / 8;
    let mut random_number = BigInt::from_bytes_be(Sign::Plus, &generate_random_bytes(prime_bytes));
    //Making sure that the number is large enough
    let bit_mask = BigInt::one() << (prime_bits - 1);
    random_number = random_number | bit_mask;
    let mut prime_candidate = random_number;
    if &prime_candidate % 2 == BigInt::zero() {
        prime_candidate = &prime_candidate + BigInt::one()
    }
    while !is_prime(&prime_candidate, &first_primes) {
        prime_candidate = &prime_candidate + 2;
    }
    prime_candidate
}

pub fn generate_keys(key_size: u16) -> (Key, Key) {
    let first_primes = primes::primes(1000);
    let prime_bits = (key_size / 2) as usize;
    let p = find_random_prime(prime_bits, &first_primes);
    let mut q = find_random_prime(prime_bits, &first_primes);
    while q == p {
        q = find_random_prime(prime_bits, &first_primes);
    }
    let public_exponent: BigInt = BigInt::from_u32(PUBLIC_EXPONENT).unwrap();

    let n: BigInt = &p * &q;
    let totient_function = (&p - 1) * (&q - 1);
    let private_exponent = find_private_key(&totient_function, &public_exponent);
    assert_eq!((&public_exponent * &private_exponent) % totient_function, BigInt::from_u8(1).unwrap());

    let public_key = Key {
        exponent: public_exponent,
        modulo: n.clone()
    };
    let private_key = Key {
        exponent: private_exponent,
        modulo: n
    };
    (public_key, private_key)
}

fn encrypt_number(number_to_encrypt: &BigInt, key: &Key) -> BigInt {
    modulo_arithmetic::exponent(number_to_encrypt, &key.exponent, &key.modulo)
}

const ENCRYPTED_CHUNK_PREFIX: u8 = 128;

fn encrypt_chunk(data: &Vec<u8>, key: &Key, modulo_size_bytes: usize) -> Vec<u8> {
    let number_to_encrypt = BigInt::from_bytes_be(Sign::Plus, &data);
    let encrypted = encrypt_number(&number_to_encrypt, key).to_bytes_be();
    let mut result_bytes = encrypted.1;
    while result_bytes.len() < modulo_size_bytes {
        result_bytes.insert(0, 0u8); // Pad the result with zeros to be exactly modulo_size_bytes
    }
    result_bytes
}

pub fn encryption_block_size(key: &Key) -> usize {
    let modulo_size_bytes = key.modulo.to_bytes_be().1.len();
    // leave one byte for ENCRYPTED_PREFIX and one byte to make sure that modulo is not overflown
    cmp::max(modulo_size_bytes - 2, 1)
}

pub fn decryption_block_size(key:&Key) -> usize {
    key.modulo.to_bytes_be().1.len()
}

pub fn encrypt_bytes(data: &Vec<u8>, key: &Key) -> Vec<u8> {
    let modulo_size_bytes = key.modulo.to_bytes_be().1.len();
    // leave one byte for ENCRYPTED_PREFIX and one byte to make sure that modulo is not overflown
    let block_size_bytes = cmp::max(modulo_size_bytes - 2, 1);
    let mut all_bytes = Vec::new();
    all_bytes.extend(data);

    let mut encrypted: Vec<u8> = Vec::new();
    for chunk in all_bytes.chunks(block_size_bytes) {
        let mut data_to_encrypt: Vec<u8> = Vec::new();
        data_to_encrypt.push(ENCRYPTED_CHUNK_PREFIX);
        data_to_encrypt.extend(chunk);
        let encrypted_chunk = encrypt_chunk(&data_to_encrypt.to_vec(), &key, modulo_size_bytes);
        encrypted.extend(encrypted_chunk);
    }
    encrypted
}

pub fn decrypt_bytes(data: &Vec<u8>, key: &Key) -> Vec<u8> {
    let modulo_size_bytes = key.modulo.to_bytes_be().1.len();
    let mut decrypted: Vec<u8> = Vec::new();
    for chunk in data.chunks(modulo_size_bytes) {
        let decrypted_data = encrypt_chunk(&chunk.to_vec(), &key, modulo_size_bytes);
        let mut i = 0;
        let mut has_found_prefix = decrypted_data[i] == ENCRYPTED_CHUNK_PREFIX;
        while !has_found_prefix && decrypted_data[i] == 0 {
            i = i + 1;
            has_found_prefix = decrypted_data[i] == ENCRYPTED_CHUNK_PREFIX;
        }
        //assert!(has_found_prefix);
        let decrypted_chunk = decrypted_data[i + 1..].to_vec();
        decrypted.extend(decrypted_chunk);
    }
    decrypted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn predefined_keys() -> (Key, Key) {
        let public_key = Key {
            exponent: BigInt::from_u32(65537).unwrap(),
            modulo: BigInt::from_u64(404790586766519).unwrap()
        };
        let private_key = Key {
            exponent: BigInt::from_u64(375946200922409).unwrap(),
            modulo: BigInt::from_u64(404790586766519).unwrap()
        };
        (public_key, private_key)
    }

    fn generated_keys() -> (Key, Key) {
        generate_keys(2048)
    }

    fn get_random_bytes(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..=255)).collect()
    }

    #[test]
    fn should_serialize_key() {
        let key = Key {
            exponent: BigInt::from_u8(2).unwrap(),
            modulo: BigInt::from_u8(13).unwrap()
        };
        assert_eq!(key.as_bytes(), vec![0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 1u8, 2u8, 13u8])
    }

    #[test]
    fn should_deserialize_key() {
        let key_bytes = vec![0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 2u8, 2u8, 128u8, 255u8];
        assert_eq!(Key::from_bytes(&key_bytes).unwrap(), Key {
            exponent: BigInt::from_u8(2).unwrap(),
            modulo: BigInt::from_u16(33023).unwrap()
        })
    }

    #[test]
    fn should_find_private_key() {
        assert_eq!(find_private_key(&BigInt::from_u32(3120).unwrap(), &BigInt::from_u32(17).unwrap()), BigInt::from_u32(2753).unwrap())
    }

    #[test]
    fn should_encrypt_and_decrypt_a_number_using_predefined_keys() {
        let (public_key, private_key) = predefined_keys();
        let original_number = BigInt::from_u64(65).unwrap();
        let encrypted = encrypt_number(&original_number, &public_key);
        let decrypted = encrypt_number(&encrypted, &private_key);
        assert_eq!(original_number, decrypted);
    }

    #[test]
    fn should_encrypt_and_decrypt_a_vec_of_bytes_using_predefined_keys() {
        let (public_key, private_key) = predefined_keys();
        let input: Vec<u8> = get_random_bytes(1000);
        let encrypted = encrypt_bytes(&input, &public_key);
        let decrypted = decrypt_bytes(&encrypted, &private_key);
        assert_eq!(input, decrypted);
    }

    #[test]
    fn should_encrypt_and_decrypt_a_string_using_predefined_keys() {
        let (public_key, private_key) = predefined_keys();
        let text = "The quick brown fox jumps over the lazy dog";
        let encrypted = encrypt_bytes(&text.as_bytes().to_vec(), &public_key);
        let decrypted = decrypt_bytes(&encrypted, &private_key);
        let decrypted_text = String::from_utf8_lossy(&decrypted);
        assert_eq!(text, decrypted_text);
    }

    #[test]
    fn should_encrypt_and_decrypt_a_number_using_generated_keys() {
        let (public_key, private_key) = generated_keys();
        let original_number = BigInt::from_u64(4093350987293047).unwrap();
        let encrypted = encrypt_number(&original_number, &public_key);
        let decrypted = encrypt_number(&encrypted, &private_key);
        assert_eq!(original_number, decrypted);
    }

    #[test]
    fn should_encrypt_and_decrypt_a_vec_of_bytes_using_generated_keys() {
        let (public_key, private_key) = generated_keys();
        let input: Vec<u8> = get_random_bytes(1000);
        let encrypted = encrypt_bytes(&input, &public_key);
        let decrypted = decrypt_bytes(&encrypted, &private_key);
        assert_eq!(input, decrypted);
    }

    #[test]
    fn should_encrypt_and_decrypt_a_string_using_generated_keys() {
        let (public_key, private_key) = generated_keys();
        let text = "The quick brown fox jumps over the lazy dog";
        let encrypted = encrypt_bytes(&text.as_bytes().to_vec(), &public_key);
        let decrypted = decrypt_bytes(&encrypted, &private_key);
        let decrypted_text = String::from_utf8_lossy(&decrypted);
        assert_eq!(text, decrypted_text);
    }
}