#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    };

    let gcd = gcd(A, gcd(B, C));
    let ans = (A + B + C) / gcd - 3;
    println!("{}", ans);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
