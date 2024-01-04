#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        S: String
    };

    let hash_set: HashSet<_> = S.chars().collect();
    let len = hash_set.len();

    let ans = factorial(len);
    println!("{}", ans)
}

fn factorial(num: usize) -> usize {
    match num {
        3 => 6,
        2 => 3,
        _ => 1,
    }
}
