#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize);M]
    }

    let mut G = vec![vec![]; N + 1];

    for (a, b) in AB {
        G[a].push(b);
        G[b].push(a);
    }

    for (i, g) in G.iter().enumerate().skip(1) {
        let s = g
            .iter()
            .sorted_unstable()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        println!("{}: {{{}}}", i, s)
    }
}
