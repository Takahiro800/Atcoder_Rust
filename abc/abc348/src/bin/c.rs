#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        AC: [(usize,usize);N]
    };

    let mut map = HashMap::new();

    for &(a, c) in AC.iter() {
        let p = map.entry(c).or_insert(std::usize::MAX);
        *p = (*p).min(a);
    }

    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
