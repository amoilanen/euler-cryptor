use crate::euclidean;

pub(crate) struct Key {
    pub(crate) exponent: u64,
    pub(crate) modulo: u64
}

fn find_private_key(totient_function: i64, public_key: i64) -> i64 {
  let gcd_and_coefficients = euclidean::find_gcd_and_bezout_coefficients(public_key, totient_function);
  let mut private_key = gcd_and_coefficients.y;
  if private_key < 0 {
    private_key = private_key + totient_function;
  }
  private_key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_private_key() {
        assert_eq!(find_private_key(3120, 17), 2753)
    }
}