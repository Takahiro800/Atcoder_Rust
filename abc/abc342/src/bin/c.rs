#![allow(non_snake_case)]
use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _N: usize,
        mut S: Chars,
        Q: usize,
        CD: [(Chars, Chars); Q]
    };
    let mut map = HashMap::new();

    for a in 'a'..='z' {
        map.insert(a, a);
    }

    for (s, t) in CD {
        let (s, t) = (s[0], t[0]);

        for v in map.values_mut() {
            if *v == s {
                *v = t;
            }
        }
    }

    let ans = S.iter().map(|c| map[c]).join("");
    println!("{}", ans);
}
