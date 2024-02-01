#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: u32,
        B: u32,
    };

    let ans = A.pow(B) + B.pow(A);

    println!("{}", ans)
}
