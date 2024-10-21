use num_bigint::BigInt;
use num_traits::{FromPrimitive, One, Zero};

pub(crate) fn exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
    let zero = BigInt::zero();
    let one = BigInt::one();
    let mut remaining_power = power.clone();
    let mut result: BigInt = BigInt::from_u8(1).unwrap();
    let mut number_to_exponentiate: BigInt = number.clone() % modulo;
    while !remaining_power.is_zero() {
        if &remaining_power & &one == zero {
            remaining_power = remaining_power / 2;
            number_to_exponentiate = (&number_to_exponentiate * &number_to_exponentiate) % modulo;
        } else {
            remaining_power = remaining_power - 1;
            result = (result * &number_to_exponentiate) % modulo;
        }
    }
    result
}