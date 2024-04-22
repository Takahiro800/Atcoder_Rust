#![allow(non_snake_case)]
use std::u64;

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    };

    let ans = a < c.pow(b);
    println!("{}", if ans { "Yes" } else { "No" });
}
