#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! { S: String };

    let n = S.len();
    let ans = (0..=n)
        .tuple_combinations()
        .filter(|&(i, j)| S[i..j].chars().eq(S[i..j].chars().rev()))
        .map(|(i, j)| j - i)
        .max()
        .unwrap();

    println!("{}", ans);
}
