#![allow(non_snake_case)]
use itertools::{iproduct, Itertools};
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        N: usize,
        S: [Chars; N],
    }

    let a = (0..N)
        .map(|i| S[i].iter().copied().filter(|&c| c == 'o').count())
        .collect_vec();
    let b = (0..N)
        .map(|j| (0..N).map(|i| S[i][j]).filter(|&c| c == 'o').count())
        .collect_vec();

    let ans = iproduct!(0..N, 0..N)
        .filter(|&(i, j)| S[i][j] == 'o')
        .map(|(i, j)| (a[i] - 1) * (b[j] - 1))
        .sum::<usize>();
    println!("{}", ans);
}
