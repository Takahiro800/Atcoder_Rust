#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        W: [usize;N]
    };

    let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (a, w) in A.iter().zip(W.iter()) {
        map.entry(*a).or_insert_with(Vec::new).push(*w);
    }
    let sum: usize = map
        .values()
        .filter(|vec| vec.len() >= 2)
        .map(|vec| {
            let max_val = vec.iter().max().unwrap_or(&0);
            vec.iter().sum::<usize>() - max_val
        })
        .sum();

    println!("{}", sum);
}
