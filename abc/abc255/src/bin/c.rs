#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        X: isize,
        A: isize,
        D: isize,
        N: usize
    };

    let min = A.min(A + D * (N as isize - 1));
    let max = A.max(A + D * (N as isize - 1));

    let ans;

    if X <= min {
        ans = min - X;
    } else if X >= max {
        ans = X - max;
    } else {
        let m = (X.abs_diff(A)) as isize / D.abs();
        ans = (m - 1..=m + 1)
            .map(|p| X.abs_diff(A + D * p) as isize)
            .min()
            .unwrap();
    }

    println!("{}", ans);
}
