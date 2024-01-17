#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        N: usize,
        A: [usize; N]
    };

    let max = A.iter().max().unwrap();
    let ans = A.iter().filter(|a| a < &max).max().unwrap();

    println!("{}", ans)
}
