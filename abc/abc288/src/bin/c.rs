#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Usize1};
// use superslice::*;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1);M]
    };

    let mut ans = 0;
    let mut dsu = UnionFind::new(N);

    for (a, b) in AB {
        if dsu.unite(a, b).is_none() {
            ans += 1;
        }
    }
    println!("{}", ans);
}

//---------- begin union_find ----------
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
    connected_group_num: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
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

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
//---------- begin union_find ----------
