#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        S: Bytes
    };
    let ans = S[0] <= b'Z' && S[1..].iter().all(|c| c >= &b'a');
    println!("{}", if ans { "Yes" } else { "No" });
}
