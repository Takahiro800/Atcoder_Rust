#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N],
        mut B: [usize;N]
    };

    A.sort();
    B.sort();

    let mut ans = 0;

    for (&a, &b) in A.iter().zip(B.iter()) {
        ans += a.abs_diff(b);
    }

    println!("{}", ans);
}
