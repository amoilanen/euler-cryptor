/*
 * Slow and inefficient "naive" implementation of the Sieve of Eratosthenes
 */
fn primes(up_to: usize) -> Vec<usize> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_first_prime_numbers() {
        assert_eq!(primes(100), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97])
    }
}