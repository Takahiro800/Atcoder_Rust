#![allow(non_snake_case)]
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        H: usize,
        W: usize,
        Q: usize,
    };

    let mut grid = vec![vec![false; W]; H];
    let mut uf = UnionFind::new(H * W);
    let dir = [(1, 0), (0, 1), (!0, 0), (0, !0)];

    for _ in 0..Q {
        input! { q: usize };

        match q {
            1 => {
                input! {
                    y: Usize1,
                    x: Usize1,
                }
                grid[y][x] = true;

                for (dx, dy) in dir {
                    let p = x.wrapping_add(dx);
                    let q = y.wrapping_add(dy);

                    if p < W && q < H && grid[q][p] {
                        uf.union(q * W + p, y * W + x);
                    }
                }
            }
            2 => {
                input! {
                    ya: Usize1,
                    xa: Usize1,
                    yb: Usize1,
                    xb: Usize1,
                }

                if !(grid[ya][xa] && grid[yb][xb]) {
                    println!("No");
                } else {
                    if uf.equiv(ya * W + xa, yb * W + xb) {
                        println!("Yes")
                    } else {
                        println!("No");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}
