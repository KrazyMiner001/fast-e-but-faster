use std::{fs, io, thread};
use std::f32::consts::LOG2_10;
use std::fs::File;
use std::time::SystemTime;

use rug::{Complete, Float, Integer, Rational};
use rug::ops::DivRounding;

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

    let start_time = SystemTime::now();

    let e = calc_e(n.trim().parse().unwrap_or(1));

    let e_decimal = Float::with_val((precision.trim().parse::<f32>().unwrap() * LOG2_10).ceil() as u32, e);

    let end_time = SystemTime::now();

    fs::write("e.txt",
              e_decimal.to_string()
    ).expect("TODO: panic message");

    println!("Calculated e with n = {} and a precision of {} in {} seconds", n, precision, end_time.duration_since(start_time).unwrap().as_secs_f64());
}

fn p(a: &Integer, b: &Integer) -> Integer {
    if b.eq(&(a + 1u8).complete()) {
        Integer::ONE.clone()
    } else {
        let m = &(a + b).complete().div_floor(2);
        p(a, m) * q(m, b) + p(m, b)
    }
}

fn q(a: &Integer, b: &Integer) -> Integer {
    if b.eq(&(a + 1u8).complete()) {
        b.clone()
    } else {
        let m = &(a + b).complete().div_floor(2);
        q(a, m) * q(m, b)
    }
}

fn calc_e(n: u32) -> Rational {
    let (top, bottom) = thread::scope(|scope| {
        let top_thread = scope.spawn(|| p(&Integer::ZERO, &n.into()));
        let bottom_thread = scope.spawn(|| q(&Integer::ZERO, &n.into()));

        (top_thread.join().unwrap(), bottom_thread.join().unwrap())
    });

    let top: Rational = top.into();
    let bottom: Rational = bottom.into();

    1 + (top / bottom)
}
