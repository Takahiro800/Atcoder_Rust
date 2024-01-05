#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [[usize;2];M]
    };

    let looser: HashSet<_> = AB.iter().map(|ab| ab[1]).collect();

    if looser.len() == N - 1 {
        let ans: usize = N * (N + 1) / 2 - looser.iter().sum::<usize>();
        println!("{}", ans)
    } else {
        println!("-1")
    }
}
