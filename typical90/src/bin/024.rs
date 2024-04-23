#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize;N],
        B: [usize;N]
    };

    let diff: usize = A.iter().zip(B.iter()).map(|(&a, &b)| a.abs_diff(b)).sum();
    let ans = K >= diff && (K - diff) % 2 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
