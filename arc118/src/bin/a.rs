#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        t: usize,
        N: usize
    };

    let ans = ((N * 100) + t - 1) / t + (N - 1);
    println!("{}", ans);
}
