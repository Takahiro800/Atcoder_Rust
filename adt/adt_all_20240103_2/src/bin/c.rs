#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        ab: [[usize;2]; N-1]
    };

    let mut map = HashMap::new();
    for &num in ab.iter().flatten() {
        *map.entry(num).or_insert(0) += 1;
    }

    let ans = map.values().clone().max().unwrap();
    if *ans == (N - 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
