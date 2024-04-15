#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        H: usize,
        X: usize,
        P: [usize;N]
    };

    let diff = X - H;
    let ans = P.lower_bound(&diff) + 1;
    println!("{}", ans);
}
