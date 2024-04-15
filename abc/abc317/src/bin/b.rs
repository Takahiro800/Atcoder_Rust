#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };

    A.sort();
    let s = A[0];
    let e = A.last().unwrap();

    let sum = (N + 1) * (s + e) / 2;
    let ans = sum - A.iter().sum::<usize>();
    println!("{}", ans);
}
