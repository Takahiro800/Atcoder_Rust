#![allow(non_snake_case)]
use num_integer::Integer;
use proconio::input;

fn main() {
    input! {
        A: usize,
        B: usize,
        C: usize,
    };

    let gcd = A.gcd(&B.gcd(&C));
    let ans = (A + B + C) / gcd - 3;
    println!("{}", ans);
}
