#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        S: Bytes
    };

    let ans = (0..16).filter(|i| *i % 2 == 1).all(|i| S[i] == b'0');
    println!("{}", if ans { "Yes" } else { "No" });
}
