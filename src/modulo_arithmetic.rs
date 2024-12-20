use num_bigint::BigInt;
use num_traits::{One, Zero};
use crate::euclidean;

pub(crate) fn exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
    fast_exponent(number, power, modulo)
}

fn fast_exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
    let optimization = MontgomeryOptimization::for_modulo(&modulo);
    let mut result: BigInt = optimization.to_montgomery_form(&BigInt::one());
    let mut number_to_exponentiate: BigInt = optimization.to_montgomery_form(number);
    for i in 0..power.bits() {
        if power.bit(i) {
            result = optimization.montgomery_form_multiply(&result, &number_to_exponentiate);
        }
        number_to_exponentiate = optimization.montgomery_form_multiply(&number_to_exponentiate, &number_to_exponentiate);
    }
    optimization.from_montgomery_form(&result)
}

#[derive(Debug, PartialEq, Clone)]
struct MontgomeryOptimization {
    n: BigInt,
    r: BigInt,
    r_stroke: BigInt,
    n_stroke: BigInt
}

impl MontgomeryOptimization {

    fn for_modulo(n: &BigInt) -> MontgomeryOptimization {
        let r = BigInt::one() << &n.bits();
        let result = euclidean::find_gcd_and_bezout_coefficients(&r, &n);
        let r_stroke = if result.x < BigInt::zero() {
            result.x + n
        } else {
            result.x
        };
        let n_stroke = (&r * &r_stroke - 1) / n;
        MontgomeryOptimization {
            n: n.clone(),
            r,
            r_stroke,
            n_stroke
        }
    }

    // Implementation of the REDC algorithm, see https://en.wikipedia.org/wiki/Montgomery_modular_multiplication
    fn redc(&self, a: &BigInt) -> BigInt {
        let m = ((a & (&self.r - 1)) * &self.n_stroke) & (&self.r - 1);
        let t: BigInt = (a + &m * &self.n) >> &self.n.bits();
        if &t >= &self.n {
            t - &self.n
        } else {
            t
        }
    }

    fn to_montgomery_form(&self, a: &BigInt) -> BigInt {
        (a << &self.n.bits()) % &self.n
    }

    fn from_montgomery_form(&self, a: &BigInt) -> BigInt {
       self.redc(a)
    }

    fn montgomery_form_multiply(&self, a: &BigInt, b: &BigInt) -> BigInt {
        self.redc(&(a * b))
    }

    fn multiply(&self, a: &BigInt, b: &BigInt) -> BigInt {
        let x = self.to_montgomery_form(a);
        let y = self.to_montgomery_form(b);
        let result = self.montgomery_form_multiply(&x, &y);
        self.from_montgomery_form(&result)
    }
}

#[cfg(test)]
mod tests {
    use num_traits::FromPrimitive;

    use super::*;

    // Slow exponent using "% modulo" which is a slow operation for an arbitrary modulo
    fn slow_exponent(number: &BigInt, power: &BigInt, modulo: &BigInt) -> BigInt {
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

    #[test]
    fn exponent_with_montgomery_multiplication() {
        let number = BigInt::from_u64(32817698412213213).unwrap();
        let power = BigInt::from_u64(8752368742787).unwrap();
        let modulo = BigInt::from_u64(404790586766519).unwrap();

        assert_eq!(fast_exponent(&number, &power, &modulo), slow_exponent(&number, &power, &modulo))
    }

    #[test]
    fn montgomery_optimization_for_modulo() {
        let n = BigInt::from_u8(11).unwrap();
        let optimization = MontgomeryOptimization::for_modulo(&n);

        assert_eq!(optimization, MontgomeryOptimization {
            n,
            r: BigInt::from_u8(16).unwrap(),
            r_stroke: BigInt::from_u8(9).unwrap(),
            n_stroke: BigInt::from_u8(13).unwrap()
        })
    }

    #[test]
    fn redc_algorithm() {
        let n = BigInt::from_u8(11).unwrap();
        let optimization = MontgomeryOptimization::for_modulo(&n);
        let a = BigInt::from_u8(43).unwrap() % &optimization.n;
        let x = optimization.to_montgomery_form(&a);
        assert_eq!(x, (&a * &optimization.r) % &optimization.n);
        assert_eq!(optimization.from_montgomery_form(&x), a);
    }

    #[test]
    fn montgomery_multiply() {
        let a = BigInt::from_u8(3).unwrap();
        let b = BigInt::from_u8(17).unwrap();
        let n = BigInt::from_u8(11).unwrap();
        let optimization = MontgomeryOptimization::for_modulo(&n);
        assert_eq!(optimization.multiply(&a, &b), (a * b) % n)
    }

    #[test]
    fn should_exponentiate_correctly() {
        let result = exponent(&BigInt::from_u16(2).unwrap(), &BigInt::from_u16(4).unwrap(), &BigInt::from_u16(17).unwrap());
        assert_eq!(result, BigInt::from_u16(16).unwrap());
        let result = exponent(&BigInt::from_u16(2).unwrap(), &BigInt::from_u16(30).unwrap(), &BigInt::from_u64(17).unwrap());
        assert_eq!(result, BigInt::from_u32(13).unwrap())
    }
}