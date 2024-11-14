use num_bigint::{BigInt, RandBigInt};
use num_traits::{FromPrimitive, One, Zero};
use rand::thread_rng;

use crate::modulo_arithmetic;

/*
 * Slow and inefficient "naive" implementation of the Sieve of Eratosthenes
 */
pub(crate) fn primes(up_to: usize) -> Vec<usize> {
    let mut is_prime: Vec<bool> = vec![true; up_to + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut current_number = 2;
    while current_number <= up_to {
        let mut proven_as_not_prime = current_number * current_number;
        while proven_as_not_prime <= up_to {
            is_prime[proven_as_not_prime] = false;
            proven_as_not_prime = proven_as_not_prime + current_number;
        }
        current_number = current_number + 1;
        while current_number <= up_to && !is_prime[current_number] {
            current_number = current_number + 1;
        }
    }
    let mut primes: Vec<usize> = Vec::new();
    for (current_number, &is_current_prime) in is_prime.iter().enumerate() {
        if is_current_prime {
            primes.push(current_number);
        }
    }
    primes
}

pub(crate) fn primes_segment(from: usize, to: usize) -> Vec<usize> {
    let largest_prime_to_cross_over = (to as f64).sqrt().ceil() as usize;
    let primes_to_cross_over = primes(largest_prime_to_cross_over);
    let segment_size = to - from + 1;
    let mut is_prime: Vec<bool> = vec![true; segment_size];
    for prime in primes_to_cross_over {
        let start_in_segment = from % prime;
        let mut prime_multiple_index_in_segment = if start_in_segment == 0 {
            0
        } else {
            prime - start_in_segment
        };
        while prime_multiple_index_in_segment < segment_size {
            is_prime[prime_multiple_index_in_segment] = false;
            prime_multiple_index_in_segment = prime_multiple_index_in_segment + prime;
        }
    }
    let mut primes: Vec<usize> = Vec::new();
    for (index_in_segment, &is_current_prime) in is_prime.iter().enumerate() {
        if is_current_prime {
            primes.push(from + index_in_segment);
        }
    }
    primes
}

/*
 * ~75% of "bases": numbers smaller than n disprove that n is prime when used in the test.
 * 0.25 probability not to detect that a number is not prime using a single base
 * then 0.25^50 = 7.888609052210118e-31 probability not to detect that a number is not prime, practically 0
 */
const NUM_OF_BASES_TO_TRY: u8 = 50;

pub(crate) fn miller_rabin_primality_test(n: &BigInt) -> bool {
    let mut rng = thread_rng();
    if n % 2 == BigInt::zero() {
        return false
    }
    // a ^ (n - 1) = 1 (mod n) for prime n (Fermat's Little Theorem)
    let mut s = 0;
    let mut d: BigInt = n - 1;
    let two = BigInt::one() << 1;
    while &d % 2 == BigInt::zero() {
        s = s + 1;
        d = d / &two;
    }
    let mut passed_check: bool = true;
    let mut bases_to_try = NUM_OF_BASES_TO_TRY;

    while passed_check && bases_to_try > 0 {
        bases_to_try = bases_to_try - 1;
        let base = rng.gen_bigint_range(&BigInt::from(2), &(n - &BigInt::from(1)));
        let mut base_exponent = modulo_arithmetic::exponent(&base, &d, &n);

        // a ^ d != 1 (mod p)
        if base_exponent != BigInt::one() {
            let mut r = 0;
            // a ^ (2 ^ r) ^ d != -1 (mod p)
            while base_exponent != n - 1 && r < s {
                base_exponent = (&base_exponent * &base_exponent) % n;
                r = r + 1;
            }
            if r == s {
                passed_check = false;
            }
        }
    }
    passed_check
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_prime_numbers() {
        assert_eq!(primes(100), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97])
    }

    #[test]
    fn should_find_primes_in_segment() {
        assert_eq!(primes_segment(10, 20), vec![11, 13, 17, 19]);
        assert_eq!(primes_segment(40, 50), vec![41, 43, 47]);
        assert_eq!(primes_segment(80, 100), vec![83, 89, 97]);
        assert_eq!(primes_segment(10000000000000, 10000000000100), vec![10000000000037, 10000000000051, 10000000000099]);
        //Would already be too slow, better to use a more optimized approach, such as the Miller-Rabin primality test
        //assert_eq!(primes_segment(1000000000000000000, 1000000000000001000), vec![]);
    }

    #[test]
    fn miller_rabin_primality_test_should_pass_for_known_prime() {
        let prime = 83;
        assert!(miller_rabin_primality_test(&BigInt::from_usize(prime).unwrap()))
    }

    #[test]
    fn miller_rabin_primality_test_should_fail_for_known_composite() {
        let n = 55;
        assert!(!miller_rabin_primality_test(&BigInt::from_usize(n).unwrap()))
    }

    #[test]
    fn miller_rabin_primality_test_should_pass_for_prime_numbers() {
        let from = 10000;
        let to = 11000;
        let primes = primes_segment(from, to);
        for prime in primes {
            assert!(miller_rabin_primality_test(&BigInt::from_usize(prime).unwrap()))
        }
    }

    #[test]
    fn miller_rabin_primality_test_should_fail_for_composite_numbers() {
        let from = 10000;
        let to = 11000;
        let primes = primes_segment(from, to);
        for n in (from..to).into_iter() {
            if !primes.contains(&n) {
                assert!(!miller_rabin_primality_test(&BigInt::from_usize(n).unwrap()))
            }
        }
    }
}