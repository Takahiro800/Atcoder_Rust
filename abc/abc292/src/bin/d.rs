#![allow(non_snake_case)]
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1,Usize1);M]
    };

    let mut uf = UnionFind::new(N + 1);
    for &(i, j) in UV.iter() {
        if !uf.union(i, j) {
            uf.union(i, N);
        }
    }

    let ans = N == M && (0..N).all(|i| uf.equiv(i, N));
    println!("{}", if ans { "Yes" } else { "No" });
}
