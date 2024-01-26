#![allow(non_snake_case)]
use itertools::*;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        S: [Chars; N]
    };

    let mut counts: Vec<(usize, usize)> = vec![];

    for (i, s) in S.iter().enumerate() {
        let count = s.iter().filter(|&c| c == &'o').count();
        counts.push((i, count))
    }

    counts.sort_by(|a, b| match a.1.cmp(&b.1) {
        std::cmp::Ordering::Equal => a.0.cmp(&b.0).reverse(),
        other => other,
    });
    counts.reverse();

    let ans: Vec<usize> = counts.iter().map(|(i, _)| *i + 1).collect();
    println!("{}", ans.iter().join(" "))
}
