#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Usize1};
// use superslice::*;

fn main() {
    input! {
        N: usize,
        M: usize,
        UV: [(Usize1, Usize1); M]
    };

    let mut deg = vec![0; N];
    let mut dsu = DSU::new(N);

    for (a, b) in UV {
        deg[a] += 1;
        deg[b] += 1;
        dsu.unite(a, b);
    }

    let ans = dsu.connected_group_num == 1
        && deg.iter().all(|d| *d <= 2)
        && deg.iter().filter(|d| **d == 1).count() == 2;

    println!("{}", if ans { "Yes" } else { "No" });
}

//---------- begin union_find ----------
#[allow(clippy::module_inception)]
struct DSU {
    par: Vec<usize>,
    siz: Vec<usize>,
    connected_group_num: usize,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            par: (0..n).collect(),
            siz: vec![1; n],
            connected_group_num: n,
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }

        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> Option<(usize, usize)> {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return None;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        self.connected_group_num -= 1;
        Some((parent, child))
    }
}
//---------- begin union_find ----------
