#![allow(non_snake_case)]
use itertools::*;
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let s = S.chars().join(" ");

    println!("{}", s)
}
