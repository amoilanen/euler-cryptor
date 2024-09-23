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

fn primes_segment(from: usize, to: usize) -> Vec<usize> {
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
        //Would already be too slow, better to use a more optimized approach, such as the AKS primality test
        //assert_eq!(primes_segment(1000000000000000000, 1000000000000001000), vec![]);
    }
}