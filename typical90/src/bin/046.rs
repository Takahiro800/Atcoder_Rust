#![allow(non_snake_case)]
use itertools::{iproduct, Itertools};
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [[usize; N]; 3],
    };
    const M: usize = 46;

    let a: Vec<Vec<usize>> = A
        .iter()
        .map(|a| {
            let mut b = [0; M];
            a.iter().for_each(|&x| b[x % M] += 1);
            b.to_vec()
        })
        .collect_vec();

    let ans = iproduct!(0..M, 0..M, 0..M)
        .filter(|&(i, j, k)| (i + j + k) % M == 0)
        .map(|(i, j, k)| a[0][i] * a[1][j] * a[2][k])
        .sum::<usize>();

    println!("{}", ans);
}
