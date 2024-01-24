#![allow(non_snake_case)]
use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Bytes; H],
    }
    let mut dsu = DSU::new(H * W);
    let mut ans = S.iter().flatten().filter(|c| **c == b'#').count();
    for i in 0..H {
        for j in 0..W {
            if S[i][j] != b'#' {
                continue;
            }
            for &(dx, dy) in [(0, 1), (1, 1), (1, 0), (1, !0)].iter() {
                let x = i.wrapping_add(dx);
                let y = j.wrapping_add(dy);
                if x < H && y < W && S[x][y] == b'#' && dsu.unite(i * W + j, x * W + y).is_some() {
                    ans -= 1;
                }
            }
        }
    }
    println!("{}", ans);
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
