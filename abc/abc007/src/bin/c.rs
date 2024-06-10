#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        H: usize,
        W: usize,
        Start: (Usize1, Usize1),
        Goarl: (Usize1, Usize1),
        Grid: [Bytes;H]
    };

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist = vec![vec![-1; W]; H];
    queue.push_back(Start);
    dist[Start.0][Start.1] = 0;

    let dir = [(1, 0), (0, 1), (!0, 0), (0, !0)];

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for &(dx, dy) in dir.iter() {
            let p = x.wrapping_add(dx);
            let q = y.wrapping_add(dy);

            if p >= H || q >= W {
                continue;
            }

            if Grid[p][q] == b'#' {
                continue;
            }

            if dist[p][q] != -1 {
                continue;
            }

            queue.push_back((p, q));
            dist[p][q] = dist[x][y] + 1;
        }
    }
    println!("{}", dist[Goarl.0][Goarl.1]);
}
