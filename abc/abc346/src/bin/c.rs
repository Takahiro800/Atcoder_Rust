#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N:usize,
        K:usize,
        mut A: [usize;N]
    };

    A.sort();
    A.dedup();
    let p = A.lower_bound(&(K + 1));
    let sum = K * (K + 1) / 2;
    let s: usize = A[..p].iter().sum();
    let ans = sum - s;
    println!("{}", ans);
}
