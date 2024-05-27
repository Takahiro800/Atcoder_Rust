#![allow(non_snake_case)]
use proconio::{
    input,
    marker::{Bytes, Usize1},
};
use std::collections::VecDeque;

fn main() {
    input! {
        H: usize,
        W: usize,
        Start: (Usize1, Usize1),
        Goal: (Usize1, Usize1),
        S: [Bytes;H],
    };

    let inf = 1000000_usize;
    let mut dp = vec![vec![inf; W]; H];
    dp[Start.0][Start.1] = 0;

    let mut queue = VecDeque::new();
    queue.push_back(Start);

    while let Some((x, y)) = queue.pop_front() {
        let d = dp[x][y] + 1;

        for &(dx, dy) in [(1, 0), (!0, 0), (0, 1), (0, !0)].iter() {
            let mut x = x + dx;
            let mut y = y + dy;

            while x < H && y < W && S[x][y] == b'.' {
                if dp[x][y] < d {
                    break;
                }

                if dp[x][y] > d {
                    dp[x][y] = d;
                    queue.push_back((x, y));
                }
                x += dx;
                y += dy;
            }
        }
    }

    println!("{}", dp[Goal.0][Goal.1] - 1);
}
