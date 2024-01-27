#![allow(non_snake_case)]
use itertools::*;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        N: usize,
        S: [Bytes; N]
    };

    let mut ord: Vec<usize> = (0..N).collect();
    ord.sort_by_cached_key(|i| !S[*i].iter().filter(|c| **c == b'o').count());
    println!("{}", ord.iter().map(|i| *i + 1).join(" "))
}
