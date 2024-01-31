#![allow(non_snake_case)]
use std::{
    collections::{HashMap, HashSet},
    usize,
};

// use itertools::*;
use proconio::{input, marker::Chars};
// use superslice::*;

fn main() {
    input! {
        N: usize,
        S: [Chars;N]
    };

    let mut map: HashMap<usize, HashSet<_>> = HashMap::new();

    for i in 0..10 {
        map.insert(i, HashSet::new());
    }

    for s in S {
        for (mut i, c) in s.iter().enumerate() {
            let num = *c as usize - '0' as usize;
            if let Some(set) = map.get_mut(&num) {
                while !set.insert(i) {
                    i += 10;
                }
            }
        }
    }

    let ans = map
        .values()
        .map(|set| set.iter().max().unwrap())
        .min()
        .unwrap();
    println!("{}", ans);
}
