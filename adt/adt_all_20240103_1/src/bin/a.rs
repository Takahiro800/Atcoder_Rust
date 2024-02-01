#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: u32,
        B: u32
    };

    println!("{}", 32u32.pow(A - B))
}
