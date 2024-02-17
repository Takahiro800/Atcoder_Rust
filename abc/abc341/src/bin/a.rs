#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let ans = "10".repeat(N) + "1";
    println!("{}", ans);
}
