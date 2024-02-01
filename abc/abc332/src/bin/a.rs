#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: usize,
        K: usize,
        PQ: [(usize, usize);N]
    };

    let mut sum = 0;

    for (price, count) in &PQ {
        sum += price * count
    }

    if sum < S {
        sum += K;
    }
    println!("{}", sum)
}
