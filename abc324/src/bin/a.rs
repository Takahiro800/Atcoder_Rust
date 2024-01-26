#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };

    A.dedup();
    let ans = A.len() == 1;
    println!("{}", if ans { "Yes" } else { "No" });
}
