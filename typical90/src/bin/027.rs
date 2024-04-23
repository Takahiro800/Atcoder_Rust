#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [String;N]
    };
    let mut set = HashSet::new();

    for (i, s) in S.iter().enumerate() {
        if set.insert(s) {
            println!("{}", i + 1);
        }
    }
}
