#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut S: Bytes
    };

    let x = S.windows(2).position(|s| s[0] != s[1]).unwrap();
    let ans = if x == 0 && S[1] == S[2] { 0 } else { x + 1 } + 1;

    println!("{}", ans);
}
