use num_bigint::{BigInt, Sign};
use num_traits::{FromPrimitive, Zero};
use std::cmp;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::euclidean;
use crate::primes;

#[derive(Debug)]
pub(crate) struct Key {
    pub(crate) exponent: BigInt,
    pub(crate) modulo: BigInt
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

pub(crate) fn generate_keys() -> (Key, Key) {
    //TODO: This is just an initial version, to be more secure the primes would have to be selected randomly from a range  > Math.pow(2, 64) - Math.pow(2, 128)
    //i.e. current primes are too small to use in a real encryption scenario
    let primes_bottom: usize = 16777216; //2^24
    let primes_top: usize = 4294967296; //2^32

    let mut rng = rand::thread_rng();
    let mut primes_from = rng.gen_range(primes_bottom + 1..primes_top);
    let segment_size = 1000;
    let p = BigInt::from_usize(primes::primes_segment(primes_from, primes_from + segment_size).choose(&mut rng).unwrap().clone()).unwrap();
    let mut q: BigInt = p.clone();
    while q == p {
        primes_from = rng.gen_range(primes_bottom + 1..primes_top);
        let new_q = BigInt::from_usize(primes::primes_segment(primes_from, primes_from + segment_size).choose(&mut rng).unwrap().clone()).unwrap();
        q = new_q;
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
    let modulo = &key.modulo;
    let mut exponent = key.exponent.clone();
    let mut result: BigInt = BigInt::from_u8(1).unwrap();
    let mut number_to_exponentiate: BigInt = number_to_encrypt.clone();
    while !exponent.is_zero() {
        if &exponent % 2 == BigInt::from_u8(0).unwrap() {
            exponent = exponent / 2;
            number_to_exponentiate = (&number_to_exponentiate * &number_to_exponentiate) % modulo;
        } else {
            exponent = exponent - 1;
            result = (result * &number_to_exponentiate) % modulo;
        }
    }
    result
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

pub(crate) fn encrypt_bytes(data: &Vec<u8>, key: &Key) -> Vec<u8> {
    let modulo_size_bytes = ((key.modulo.bits() + 7) / 8) as usize;
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

pub(crate) fn decrypt_bytes(data: &Vec<u8>, key: &Key) -> Vec<u8> {
    let modulo_size_bytes = ((key.modulo.bits() + 7) / 8) as usize;
    let mut decrypted: Vec<u8> = Vec::new();
    for chunk in data.chunks(modulo_size_bytes) {
        let decrypted_data = encrypt_chunk(&chunk.to_vec(), &key, modulo_size_bytes);
        let mut i = 0;
        let mut has_found_prefix = decrypted_data[i] == ENCRYPTED_CHUNK_PREFIX;
        while !has_found_prefix && decrypted_data[i] == 0 {
            i = i + 1;
            has_found_prefix = decrypted_data[i] == ENCRYPTED_CHUNK_PREFIX;
        }
        assert!(has_found_prefix);
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
        generate_keys()
    }

    fn get_random_bytes(size: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..=255)).collect()
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