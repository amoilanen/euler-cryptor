use num_bigint::BigInt;
use num_traits::FromPrimitive;

//gcd = a * x + b * y
#[derive(Debug, PartialEq)]
pub(crate) struct GcdAndBezoutCoefficients {
    pub(crate) a: BigInt,
    pub(crate) b: BigInt,
    pub(crate) gcd: BigInt,
    pub(crate) x: BigInt,
    pub(crate) y: BigInt
}

pub(crate) fn find_gcd_and_bezout_coefficients(first: &BigInt, second: &BigInt) -> GcdAndBezoutCoefficients {
    /*
     * At each step: a = b * q + r
     * 
     * if a0 and b0 are original numbers before the first iteration:
     * 
     * a = a0​ * x0​ + b0​ * y0​
     * b = a0 * x1 + b0 * y1
     * 
     * The coefficients x0, y0, x1, y1 are recomputed as follows after every iteration:
     * 
     * r = a - b * q = a0 * (x0 - x1 * q) + b0 * (y0 - y1 * q)
     * 
     * =>
     * x0 = x1
     * y0 = y1
     * x1 = x0 - x1 * q
     * y1 = y0 - y1 * q
     */
    let mut x0: BigInt = BigInt::from_u16(1).unwrap();
    let mut y0: BigInt = BigInt::from_u16(0).unwrap();
    let mut x1: BigInt = BigInt::from_u16(0).unwrap();
    let mut y1: BigInt = BigInt::from_u16(1).unwrap();
    let mut a = if first > second { first.clone() } else { second.clone() };
    let mut b = if first > second { second.clone() } else { first.clone() };
    let mut r;
    let mut q;
    while b != BigInt::from_u8(0).unwrap() {
        r = &a % &b;
        q = &a / &b;
        a = b;
        b = r;
        let old_x1 = x1.clone();
        let old_y1 = y1.clone();
        x1 = &x0 - &x1 * &q;
        y1 = &y0 - &y1 * &q;
        x0 = old_x1.clone();
        y0 = old_y1.clone();
    }
    GcdAndBezoutCoefficients {
        a: first.clone(),
        b: second.clone(),
        gcd: a,
        x: x0,
        y: y0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_gcd_and_bezout_coefficients() {
        assert_eq!(find_gcd_and_bezout_coefficients(&BigInt::from_u32(3120).unwrap(), &BigInt::from_u32(17).unwrap()), GcdAndBezoutCoefficients {
            a: BigInt::from_u32(3120).unwrap(),
            b: BigInt::from_u32(17).unwrap(),
            gcd: BigInt::from_u32(1).unwrap(),
            x: BigInt::from_u32(2).unwrap(),
            y: BigInt::from_i32(-367).unwrap()
        })
    }
}