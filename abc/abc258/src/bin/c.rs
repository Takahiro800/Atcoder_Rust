#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Chars};
// use superslice::*;

fn main() {
    input! {
        N: usize,
        Q: usize,
        S: Chars
    };

    let mut count = 0;

    for _ in 0..Q {
        input! { q: usize }
        input! { mut i: usize }
        match q {
            1 => count += i,
            2 => {
                i = (i + N - (count % N) - 1) % N;
                println!("{}", S[i])
            }
            _ => unreachable!(),
        }
    }
}
