#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Bytes};
use std::collections::HashSet;
// use superslice::*;

fn main() {
    input! {
        _N: usize,
        M: usize,
        mut H: i32,
        K: i32,
        S: Bytes,
        XY: [(i32,i32);M]
    };

    let mut set = XY.into_iter().collect::<HashSet<_>>();
    let mut p: (i32, i32) = (0, 0);
    let mut ans = true;

    for s in S {
        H -= 1;
        if H < 0 {
            ans = false;
        }
        match s {
            b'R' => p.0 += 1,
            b'L' => p.0 -= 1,
            b'U' => p.1 += 1,
            b'D' => p.1 -= 1,
            _ => unreachable!(),
        };
        if H < K && set.contains(&p) {
            H = K;
            set.remove(&p);
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
