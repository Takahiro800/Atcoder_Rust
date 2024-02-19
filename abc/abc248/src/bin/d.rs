#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        Q: usize,
    };

    let mut map = HashMap::new();
    for (i, a) in A.iter().enumerate() {
        map.entry(a).or_insert_with(Vec::new).push(i);
    }

    for v in map.values_mut() {
        v.sort();
    }

    for _ in 0..Q {
        input! {
            L: Usize1,
            R: Usize1,
            X: usize,
        }

        if let Some(v) = map.get(&X) {
            let min = v.lower_bound(&L);
            let max = v.upper_bound(&R);
            println!("{}", max - min);
        } else {
            println!("{}", 0);
        }
    }
}
