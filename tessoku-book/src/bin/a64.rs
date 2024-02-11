#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        N: usize,
        M: usize,
        ABC: [(Usize1, Usize1, isize);M]
    }

    let mut g = vec![vec![]; N];

    for (a, b, c) in ABC {
        g[a].push((b, c));
        g[b].push((a, c));
    }

    let mut dp = vec![std::isize::MAX; N];
    dp[0] = 0;
    let mut heap = BinaryHeap::new();

    heap.push(Reverse((0, 0)));

    while let Some(Reverse((d, p))) = heap.pop() {
        if d > dp[p] {
            continue;
        }

        for (i, dis) in &g[p] {
            if d + dis < dp[*i] {
                dp[*i] = d + dis;
                heap.push(Reverse((dp[*i], *i)));
            }
        }
    }

    for a in dp {
        println!("{}", if a == std::isize::MAX { -1 } else { a });
    }
}
