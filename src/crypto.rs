use num_bigint::BigInt;
use num_traits::FromPrimitive;

use crate::euclidean;

pub(crate) struct Key {
    pub(crate) exponent: BigInt,
    pub(crate) modulo: BigInt
}

pub(crate) fn find_private_key(totient_function: &BigInt, public_key: &BigInt) -> BigInt {
  let gcd_and_coefficients = euclidean::find_gcd_and_bezout_coefficients(public_key, totient_function);
  let mut private_key = gcd_and_coefficients.y;
  if private_key < BigInt::from_u8(0).unwrap() {
    private_key = private_key + totient_function;
  }
  private_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_private_key() {
        assert_eq!(find_private_key(&BigInt::from_u32(3120).unwrap(), &BigInt::from_u32(17).unwrap()), BigInt::from_u32(2753).unwrap())
    }
}