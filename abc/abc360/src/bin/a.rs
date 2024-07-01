#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars
    };
    let r_indices: Vec<usize> = S
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'R')
        .map(|(i, _)| i)
        .collect();
    let m_indices: Vec<usize> = S
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == 'M')
        .map(|(i, _)| i)
        .collect();

    let ans = r_indices < m_indices;
    println!("{}", if ans { "Yes" } else { "No" });
}
