#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        AC: [(usize,usize);N]
    };

    let mut map = HashMap::new();

    for (a, c) in AC.iter() {
        map.entry(c)
            .and_modify(|e: &mut usize| *e = (*e).min(*a))
            .or_insert(*a);
    }

    let ans = map.values().max().unwrap();
    println!("{}", ans);
}
