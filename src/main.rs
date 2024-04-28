mod division;

use std::{fs, io, thread};
use std::fs::File;
use std::io::Write;

use bigdecimal::{num_bigint::ToBigInt, BigDecimal};
use division::impl_division;
use num::{integer::div_floor, BigInt, BigUint, One, Zero};

fn main() {
    let n = &mut "".to_string();
    let precision = &mut "".to_string();
    println!("Enter n value");
    io::stdin()
        .read_line(n).expect("Could not read n");
    println!("Enter precision");
    io::stdin()
        .read_line(precision).expect("Could not read precision");

    File::create("e.txt").expect("Could not create e.txt");
    fs::write("e.txt",
              calc_e(n.trim().parse().unwrap_or(1),
                              precision.trim().parse().unwrap_or(1)).to_string())
        .expect("Could not save to file");
}

fn p(a: &BigUint, b: &BigUint) -> BigUint {
    if b == &(a + 1u8) {
        One::one()
    } else {
        let m = &div_floor(a + b, 2u8.into());
        p(a, m) * q(m, b) + p(m, b)
    }
}

fn q(a: &BigUint, b: &BigUint) -> BigUint {
    if b == &(a + 1u8) {
        b.clone()
    } else {
        let m = &div_floor(a + b, 2u8.into());
        q(a, m) * q(m, b)
    }
}

fn calc_e(n: u32, precision: u64) -> BigDecimal {
    let ((top_int, top_scale), (bottom_int, bottom_scale)) = thread::scope(|scope| {
        let top_thread = scope.spawn(|| process_num(p(&Zero::zero(), &n.into())));
        let bottom_thread = scope.spawn(|| process_num(q(&Zero::zero(), &n.into())));

        (top_thread.join().unwrap(), bottom_thread.join().unwrap())
    });

    1 + impl_division(top_int, &bottom_int, top_scale - bottom_scale, precision)
}

fn process_num(num: BigUint) -> (BigInt, i64) {
    BigDecimal::from(num.to_bigint().unwrap()).as_bigint_and_exponent()
}
