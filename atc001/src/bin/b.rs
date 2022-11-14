#![allow(non_snake_case)]

use proconio::input;

#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.find(self.par[x]);
        self.par[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

fn main() {
    input! {
      N: usize,
      Q: usize,
      PAB_vec: [(usize, usize, usize); Q],
    };

    let mut uf = UnionFind::new(N);
    println!("{:#?}", uf);

    for (P, A, B) in PAB_vec {
        match P {
            0 => uf.unite(A, B),
            1 => {
                if uf.same(A, B) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => (),
        }
    }
}
