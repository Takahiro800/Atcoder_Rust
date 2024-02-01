#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let chars_s: Vec<_> = S.chars().collect();
    println!("{}", chars_s[S.len() / 2])
}
