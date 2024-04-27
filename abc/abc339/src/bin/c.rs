#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        N: usize,
        A: [isize;N]
    };

    let mut b = vec![0; N];
    b[0] = A[0];

    for i in 1..N {
        b[i] = A[i] + b[i - 1];
    }

    let min = b.iter().min().unwrap();
    let dif = min.min(&0).abs();
    let ans = b[N - 1] + dif;
    println!("{}", ans);
}
