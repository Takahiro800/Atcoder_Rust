#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        N: usize
    };

    let o = "o".repeat(N);
    println!("L{}ng", o);
}
