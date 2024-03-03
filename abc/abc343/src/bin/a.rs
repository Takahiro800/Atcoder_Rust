#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: usize,
        B: usize,
    };

    if A * B != 0 {
        println!("{}", 0)
    } else if A == 0 {
        println!("{}", (B + 1) % 9)
    } else {
        println!("{}", (A + 1) % 9)
    }
}
