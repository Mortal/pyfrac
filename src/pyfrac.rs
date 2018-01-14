use std::fmt;
use std::fmt::Write;
use num_bigint::BigUint;
use num_integer::Integer;
use num_rational::Ratio;
use num_traits::cast::ToPrimitive;

fn int_floorlog(n: &BigUint, base: usize) -> (isize, BigUint) {
    let mut exps = Vec::new();
    let mut exp_hi = BigUint::from(base);
    while exp_hi.le(n) {
        exps.push(exp_hi.clone());
        exp_hi *= exp_hi.clone();
    }
    // exps[i] = base ** (2**i)
    let mut exp_lo = match exps.pop() {
        Some(e) => e,
        None =>
            // base > n
            return (0, BigUint::from(1u8)),
    };
    let mut lo = 1 << exps.len();
    // exp_lo = base ** lo
    let mut hi = lo * 2;
    while hi - lo > 1 {
        // exp_lo = base ** lo
        let v = (hi - lo) / 2;
        let exp_v = exps.pop().unwrap();
        // exp_v = base ** v
        let mid = lo + v;
        let exp_mid = exp_lo.clone() * exp_v;
        if exp_mid.le(n) {
            lo = mid;
            exp_lo = exp_mid;
        } else {
            hi = mid;
        }
    }
    // base ** lo <= n < base ** (lo+1)
    (lo, exp_lo)
}

fn fraction_floorlog(p: &BigUint, q: &BigUint, base: usize) -> isize {
    if p.lt(q) {
        let (n, r) = q.div_mod_floor(p);
        let (inv_res, exp_inv_res) = int_floorlog(&n, base);
        if r.eq(&BigUint::from(0u8)) && exp_inv_res.eq(&n) {
            // base ** inv_res == q/p,
            // so base ** -inv_res == p/q,
            -inv_res
        } else {
            // base ** inv_res < q/p < base ** (inv_res + 1),
            // so base ** (-inv_res - 1) < p/q < base ** -inv_res
            -inv_res - 1
        }
    } else {
        let (res, _) = int_floorlog(&(p / q), base);
        res
    }
}

fn pow(mut base: BigUint, mut exp: usize) -> BigUint {
    let mut r = BigUint::from(1u8);
    while exp > 0 {
        // r * base**exp = base**r
        if exp % 2 == 1 {
            r *= &base;
            exp -= 1;
        } else {
            base *= base.clone();
            exp /= 2;
        }
    }
    r
}

pub struct Repeated {
    digits: Vec<u8>,
    repeat_start: usize,
    exponent: isize,
}

fn to_digit(d: u8) -> char {
    (('0' as u8) + d) as char
}

impl fmt::Display for Repeated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(to_digit(self.digits[0]))?;
        f.write_char('.')?;
        for c in &self.digits[1..self.repeat_start] {
            f.write_char(to_digit(*c))?;
        }
        if self.repeat_start < self.digits.len() {
            f.write_char('(')?;
            for c in &self.digits[self.repeat_start..] {
                f.write_char(to_digit(*c))?;
            }
            f.write_char(')')?;
        }
        Ok(())
    }
}

pub fn repeated(p: BigUint, q: BigUint, base: usize, min_exp: usize) -> Repeated {
    let exponent = fraction_floorlog(&p, &q, base);
    let min_exp = min_exp as isize;
    let exponent = if -min_exp < exponent.abs() && exponent.abs() < min_exp { 0 } else { exponent };
    let n = if exponent >= 0 {
        Ratio::new(p, q * pow(BigUint::from(base), exponent as usize))
    } else {
        Ratio::new(p * pow(BigUint::from(base), -exponent as usize), q)
    };
    let mut digits = Vec::new();
    let mut s1 = n.clone();
    let mut s2 = n.clone();
    let mut repeat_start = 0;
    loop {
        let a = s1.to_integer();
        s1 = (s1 - Ratio::from_integer(a)) * Ratio::from_integer(BigUint::from(base));
        let c = s2.to_integer();
        s2 = (s2 - Ratio::from_integer(c.clone())) * Ratio::from_integer(BigUint::from(base));
        let e = s2.to_integer();
        s2 = (s2 - Ratio::from_integer(e.clone())) * Ratio::from_integer(BigUint::from(base));
        digits.push(c.to_u8().unwrap());
        digits.push(e.to_u8().unwrap());
        repeat_start += 1;
        if s1.eq(&s2) {
            break;
        }
    }
    // Period is at most P=(digits.len()-repeat_start), but it may be a divisor of P.
    s2 = s1.clone();
    let mut period = 0;
    loop {
        assert!(repeat_start+period < digits.len());
        let e = s2.to_integer();
        s2 = (s2 - Ratio::from_integer(e.clone())) * Ratio::from_integer(BigUint::from(base));
        period += 1;
        if s1.eq(&s2) {
            break;
        }
    }
    if repeat_start+period < digits.len() {
        assert_eq!(digits[repeat_start], digits[repeat_start+period],
                   "{:?} rep={} period={}", digits, repeat_start, period);
    }
    while repeat_start > 0 && digits[repeat_start-1] == digits[repeat_start+period-1] {
        digits.pop().unwrap();
        repeat_start -= 1;
    }
    assert!(repeat_start + period <= digits.len());
    digits.resize(repeat_start + period, 0);
    if period == 1 && digits[repeat_start] == 0 {
        digits.pop().unwrap();
    }
    Repeated {
        digits: digits,
        repeat_start: repeat_start,
        exponent: exponent,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn my_int_floorlog(n: u8, base: usize) -> isize {
        let (r, exp_r) = int_floorlog(&BigUint::from(n), base);
        assert!(r >= 0);
        assert_eq!(pow(BigUint::from(base), r as usize), exp_r);
        r
    }

    #[test]
    fn int_floorlog_test() {
        assert_eq!(my_int_floorlog(1, 10), 0);
        assert_eq!(my_int_floorlog(9, 10), 0);
        assert_eq!(my_int_floorlog(10, 10), 1);
        assert_eq!(my_int_floorlog(99, 10), 1);
        assert_eq!(my_int_floorlog(100, 10), 2);
    }

    #[test]
    fn fraction_floorlog_test() {
        let four_pow = pow(BigUint::from(4u8), 100);
        let three_pow = pow(BigUint::from(3u8), 100);
        assert_eq!(fraction_floorlog(&four_pow, &three_pow, 10), 12);
        assert_eq!(fraction_floorlog(&three_pow, &four_pow, 10), -13);
    }

    #[test]
    fn repeated_test() {
        let r = repeated(BigUint::from(1u8), BigUint::from(3u8), 10, 0);
        assert_eq!(r.digits, [3]);
        assert_eq!(r.repeat_start, 0);
        assert_eq!(r.exponent, -1);
    }

    #[test]
    fn repeated_test_2() {
        let r = repeated(BigUint::from(3u8), BigUint::from(4u8), 10, 5);
        assert_eq!(r.digits, [0, 7, 5]);
        assert_eq!(r.repeat_start, 3);
        assert_eq!(r.exponent, 0);
    }
}
