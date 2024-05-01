#![allow(non_snake_case)]
use std::{collections::HashSet, vec};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        _M: usize,
    };

    let mut data = vec![];
    let mut price = vec![];

    for _ in 0..N {
        input! {
            P: usize,
            C: usize,
            F: [usize;C]
        }
        price.push(P);
        let set: HashSet<_> = HashSet::from_iter(F);
        data.push(set);
    }

    for p in (0..N).permutations(2) {
        let i = p[0];
        let j = p[1];
        if price[i] >= price[j] && data[j].is_superset(&data[i]) {
            if price[i] > price[j] || data[j].len() > data[i].len() {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
