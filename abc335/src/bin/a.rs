#![allow(non_snake_case)]
use std::fmt::Pointer;

use proconio::input;

fn main() {
    input! {
        S: String
    };

    let mut chars: Vec<char> = S.chars().collect();

    if let Some(last) = chars.last_mut() {
        *last = '4';
    }
    let ans: String = chars.into_iter().collect();
    println!("{}", ans)
}
