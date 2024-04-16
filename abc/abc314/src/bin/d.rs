#![allow(non_snake_case)]
use std::{char, usize};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _N: usize,
        mut S: Chars,
        Q: usize,
        TXC: [(usize, usize, char);Q]
    };

    let mut last = std::usize::MAX;
    for (i, &(t, _, _)) in TXC.iter().enumerate() {
        if t != 1 {
            last = i;
        }
    }

    for (i, &(t, x, c)) in TXC.iter().enumerate() {
        match t {
            1 => S[x - 1] = c,
            2 => {
                if i == last {
                    S = S.iter().map(|c| c.to_lowercase().next().unwrap()).collect();
                }
            }
            3 => {
                if i == last {
                    S = S.iter().map(|c| c.to_uppercase().next().unwrap()).collect();
                }
            }
            _ => unreachable!(),
        }
    }

    let ans = S.iter().join("");
    println!("{}", ans);
}
