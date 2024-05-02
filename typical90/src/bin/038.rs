#![allow(non_snake_case)]
use proconio::input;
use std::mem::swap;

fn main() {
    input! {
        mut A: usize,
        mut B: usize
    };

    let gcd = gcd(A, B);

    if A < B {
        swap(&mut A, &mut B);
    }
    if 10_usize.pow(18) / A < B / gcd {
        println!("Large");
    } else {
        println!("{}", A / gcd * B);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
