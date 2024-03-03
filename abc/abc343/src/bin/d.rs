#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        T: usize,
    };

    let mut p = vec![0; N + 1];
    let mut map = HashMap::new();
    map.insert(0, N);

    for _ in 0..T {
        input! {
            a: usize,
            b: usize
        }

        let current = p[a];
        if let Some(v) = map.get_mut(&current) {
            if v == &1 {
                map.remove(&current);
            } else {
                *v -= 1;
            }
        }

        let after = current + b;
        *map.entry(after).or_insert(0) += 1;
        p[a] = after;

        println!("{}", map.len())
    }
}
