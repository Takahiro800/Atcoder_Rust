#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        Query: [(usize, usize, usize);Q]
    }

    let mut uf = UnionFind::new(N);
    for &(a, b, c) in Query.iter() {
        match a {
            1 => {
                uf.unite(b - 1, c - 1);
                continue;
            }
            2 => {
                if uf.is_same(b - 1, c - 1) {
                    println!("Yes")
                } else {
                    println!("No")
                }
            }
            _ => unreachable!(),
        }
    }
}

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

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        self.connected_group_num -= 1;
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[x]
    }
}
