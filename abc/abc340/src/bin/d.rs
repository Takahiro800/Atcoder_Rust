#![allow(non_snake_case)]
use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        ABX: [(isize, isize, Usize1);N-1]
    }

    let mut dp = vec![std::isize::MAX; N];
    dp[0] = 0;

    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, 0)));

    while let Some(Reverse((d, v))) = heap.pop() {
        if d > dp[v] || v == N - 1 {
            continue;
        }

        let (a, b, x) = ABX[v];
        if d + a < dp[v + 1] {
            dp[v + 1] = d + a;
            heap.push(Reverse((dp[v + 1], v + 1)));
        }

        if d + b < dp[x] {
            dp[x] = d + b;
            heap.push(Reverse((dp[x], x)));
        }
    }

    println!("{}", dp[N - 1]);
}
