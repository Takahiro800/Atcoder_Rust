#![allow(non_snake_case)]
use std::collections::HashMap;

use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        M: usize,
        S: [Chars;3],
    };

    let ans = iproduct!(
        S[0].iter().enumerate(),
        S[1].iter().enumerate(),
        S[2].iter().enumerate()
    )
    .filter(|((_, a0), (_, a1), (_, a2))| a0 == a1 && a1 == a2)
    .map(|((i0, _), (i1, _), (i2, _))| {
        let mut map = HashMap::new();

        for &i in &[i0, i1, i2] {
            *map.entry(i).or_insert(0) += 1;
        }

        map.iter()
            .map(|(key, value)| key + M * (value - 1))
            .max()
            .unwrap()
    })
    .min()
    .map_or_else(|| -1, |x| x as isize);

    println!("{}", ans);
}
