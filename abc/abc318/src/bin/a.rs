#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        P: usize
    };

    let ans = (0..N).filter(|i| i % P == M - 1).count();
    println!("{}", ans);
}
