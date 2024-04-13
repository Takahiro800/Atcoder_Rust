#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut S: Chars
    };

    S.sort();
    let s = run_length_encoding(S);
    let values: Vec<usize> = s.iter().map(|(_, value)| *value).collect();

    let mut map = HashMap::new();
    for value in values {
        *map.entry(value).or_insert(0) += 1;
    }
    let ans = map.values().all(|&x| x == 0 || x == 2);

    println!("{}", if ans { "Yes" } else { "No" });
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
