#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String
    };

    println!("{}4", &S[..S.len() - 1])
}
