use num_bigint::BigInt;
use num_traits::{One, Zero};

pub(crate) fn exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
    let one = &BigInt::one();
    let mut remaining_power = power.clone();
    let mut result: BigInt = BigInt::one();
    let mut number_to_exponentiate: BigInt = number % modulo;
    while !remaining_power.is_zero() {
        if &remaining_power & one == *one {
            remaining_power = remaining_power - 1;
            result = (&result * &number_to_exponentiate) % modulo;
        } else {
            remaining_power >>= 1;
            number_to_exponentiate = (&number_to_exponentiate * &number_to_exponentiate) % modulo;
        }
    }
    result
}