#![allow(non_snake_case)]

use itertools::*;
use proconio::input;

fn main() {
    input! {
        N: usize,
        t: String,
    };

    let mut anses = vec![];
    for q in 1..=N {
        input! { s: String }
        let common_prefix = s.chars().zip(t.chars()).take_while(|(a, b)| a == b).count();
        let common_suffix = s
            .chars()
            .rev()
            .zip(t.chars().rev())
            .take_while(|(a, b)| a == b)
            .count();
        let ans = s.len().max(t.len()) - s.len().min(t.len()) <= 1
            && common_prefix + common_suffix >= s.len().max(t.len()) - 1;
        if ans {
            anses.push(q)
        }
    }

    println!("{}", anses.len());
    println!("{}", anses.iter().join(" "))
}
