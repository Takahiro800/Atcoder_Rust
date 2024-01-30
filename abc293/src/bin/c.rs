#![allow(non_snake_case)]
use std::collections::BTreeSet;

// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[usize;W];H]
    };

    let mut set = BTreeSet::new();
    set.insert(A[0][0]);
    let ans = dfs(0, 0, &mut set, &A);
    println!("{}", ans);
}

fn dfs(x: usize, y: usize, set: &mut BTreeSet<usize>, a: &[Vec<usize>]) -> usize {
    let h = a.len();
    let w = a[0].len();
    let mut res = 0;
    if (x + 1, y + 1) == (h, w) {
        res = 1
    } else {
        if x + 1 < h && set.insert(a[x + 1][y]) {
            res += dfs(x + 1, y, set, a);
        }
        if y + 1 < w && set.insert(a[x][y + 1]) {
            res += dfs(x, y + 1, set, a);
        }
    }
    set.remove(&a[x][y]);
    res
}
