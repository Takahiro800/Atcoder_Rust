#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        S: [usize;N]
    };

    let ans: usize = S.iter().filter(|&s| s <= &X).sum();
    println!("{}", ans);
}
