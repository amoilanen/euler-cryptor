//gcd = a * x + b * y
#[derive(Debug, PartialEq)]
pub(crate) struct GcdAndBezoutCoefficients {
    pub(crate) a: i64,
    pub(crate) b: i64,
    pub(crate) gcd: i64,
    pub(crate) x: i64,
    pub(crate) y: i64
}

pub(crate) fn find_gcd_and_bezout_coefficients(first: i64, second: i64) -> GcdAndBezoutCoefficients {
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
    let mut x0: i64 = 1;
    let mut y0: i64 = 0;
    let mut x1: i64 = 0;
    let mut y1: i64 = 1;
    let mut a = if first > second { first } else { second };
    let mut b = if first > second { second } else { first };
    let mut r;
    let mut q;
    while b != 0 {
        r = a % b;
        q = a / b;
        a = b;
        b = r;
        let old_x1 = x1;
        let old_y1 = y1;
        x1 = x0 - x1 * q;
        y1 = y0 - y1 * q;
        x0 = old_x1;
        y0 = old_y1;
    }
    GcdAndBezoutCoefficients {
        a: first,
        b: second,
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
        assert_eq!(find_gcd_and_bezout_coefficients(3120, 17), GcdAndBezoutCoefficients {
            a: 3120,
            b: 17,
            gcd: 1,
            x: 2,
            y: -367
        })
    }
}