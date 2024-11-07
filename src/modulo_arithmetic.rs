use num_bigint::BigInt;
use num_traits::One;

pub(crate) fn exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
    let mut result: BigInt = BigInt::one();
    let mut number_to_exponentiate: BigInt = number % modulo;
    for i in 0..power.bits() {
        if power.bit(i) {
            result = (&result * &number_to_exponentiate) % modulo;
        }
        number_to_exponentiate = (&number_to_exponentiate * &number_to_exponentiate) % modulo;
    }
    result
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;

    use super::*;

    #[test]
    fn should_exponentiate_correctly() {
        let result = exponent(&BigInt::from_u16(2).unwrap(), &BigInt::from_u16(4).unwrap(), &BigInt::from_u16(32).unwrap());
        assert_eq!(result, BigInt::from_u16(16).unwrap());
        let result = exponent(&BigInt::from_u16(2).unwrap(), &BigInt::from_u16(30).unwrap(), &BigInt::from_u64(10000000000).unwrap());
        assert_eq!(result, BigInt::from_u32(1073741824).unwrap())
    }
}