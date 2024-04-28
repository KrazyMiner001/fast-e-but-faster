use num::{BigUint, Signed};
use std::{
    f64::consts::LOG2_10,
    ops::{Div, Neg},
};

use bigdecimal::{BigDecimal, Zero};
use num::{BigInt, Integer};

pub fn impl_division(
    mut num: BigInt,
    den: &BigInt,
    mut scale: i64,
    max_precision: u64,
) -> BigDecimal {
    // quick zero check
    if num.is_zero() {
        return BigDecimal::new(num, 0);
    }

    match (num.is_negative(), den.is_negative()) {
        (true, true) => return impl_division(num.neg(), &den.neg(), scale, max_precision),
        (true, false) => return -impl_division(num.neg(), den, scale, max_precision),
        (false, true) => return -impl_division(num, &den.neg(), scale, max_precision),
        (false, false) => (),
    }

    // shift digits until numerator is larger than denominator (set scale appropriately)
    while num < *den {
        scale += 1;
        num *= 10;
    }

    // first division
    let (mut quotient, mut remainder) = num.div_rem(den);

    // division complete
    if remainder.is_zero() {
        return BigDecimal::from((quotient, scale));
    }

    let mut precision = count_decimal_digits(&quotient);

    // shift remainder by 1 decimal;
    // quotient will be 1 digit upon next division
    remainder *= 10;

    while !remainder.is_zero() && precision < max_precision {
        let (q, r) = remainder.div_rem(den);
        quotient = quotient * 10 + q;
        remainder = r * 10;

        precision += 1;
        scale += 1;
    }

    if !remainder.is_zero() {
        // round final number with remainder
        quotient += get_rounding_term(&remainder.div(den));
    }

    let result = BigDecimal::new(quotient, scale);
    // println!(" {} / {}\n = {}\n", self, other, result);
    return result;
}

fn count_decimal_digits(int: &BigInt) -> u64 {
    count_decimal_digits_uint(int.magnitude())
}

fn count_decimal_digits_uint(uint: &BigUint) -> u64 {
    if uint.is_zero() {
        return 1;
    }
    let mut digits = (uint.bits() as f64 / LOG2_10) as u64;
    // guess number of digits based on number of bits in UInt
    let mut num = ten_to_the(digits)
        .to_biguint()
        .expect("Ten to power is negative");
    while *uint >= num {
        num *= 10u8;
        digits += 1;
    }
    digits
}

#[inline(always)]
fn get_rounding_term(num: &BigInt) -> u8 {
    if num.is_zero() {
        return 0;
    }

    let digits = (num.bits() as f64 / LOG2_10) as u64;
    let mut n = ten_to_the(digits);

    // loop-method
    loop {
        if *num < n {
            return 1;
        }
        n *= 5;
        if *num < n {
            return 0;
        }
        n *= 2;
    }

    // string-method
    // let s = format!("{}", num);
    // let high_digit = u8::from_str(&s[0..1]).unwrap();
    // if high_digit < 5 { 0 } else { 1 }
}

pub(crate) fn ten_to_the(pow: u64) -> BigInt {
    ten_to_the_uint(pow).into()
}

pub(crate) fn ten_to_the_uint(pow: u64) -> BigUint {
    if pow < 20 {
        return BigUint::from(10u64.pow(pow as u32));
    }

    // linear case of 10^pow = 10^(19 * count + rem)
    if pow < 590 {
        let ten_to_nineteen = 10u64.pow(19);

        // count factors of 19
        let (count, rem) = pow.div_rem(&19);

        let mut res = BigUint::from(ten_to_nineteen);
        for _ in 1..count {
            res *= ten_to_nineteen;
        }
        if rem != 0 {
            res *= 10u64.pow(rem as u32);
        }

        return res;
    }

    // use recursive algorithm where linear case might be too slow
    let (quotient, rem) = pow.div_rem(&16);
    let x = ten_to_the_uint(quotient);

    let x2 = &x * &x;
    let x4 = &x2 * &x2;
    let x8 = &x4 * &x4;
    let res = &x8 * &x8;

    if rem == 0 {
        res
    } else {
        res * 10u64.pow(rem as u32)
    }
}