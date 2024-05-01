#![allow(non_snake_case)]
use std::collections::HashSet;

use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        H: usize,
        W: usize,
        S: [Bytes;H],
    };

    let mut s = S
        .iter()
        .map(|s| {
            s.iter()
                .map(|&c| if c == b'#' { 2 } else { 0 })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for (i, j) in iproduct!(0..H, 0..W) {
        if s[i][j] == 0
            && ((i > 0 && s[i - 1][j] == 2)
                || (j > 0 && s[i][j - 1] == 2)
                || (i < H - 1 && s[i + 1][j] == 2)
                || (j < W - 1 && s[i][j + 1] == 2))
        {
            s[i][j] = 1;
        }
    }

    let mut used = vec![vec![false; W]; H];
    let mut ans = 1;

    for (si, sj) in iproduct!(0..H, 0..W).filter(|&(si, sj)| s[si][sj] == 0) {
        if used[si][sj] {
            continue;
        }

        let mut stack = Vec::new();
        let mut cmp = HashSet::new();
        stack.push((si, sj));

        while let Some((i, j)) = stack.pop() {
            used[i][j] = true;
            cmp.insert((i, j));

            if s[i][j] == 0 {
                for (ni, nj) in vec![
                    (i.wrapping_sub(1), j),
                    (i, j.wrapping_sub(1)),
                    (i + 1, j),
                    (i, j + 1),
                ]
                .into_iter()
                .filter(|&(ni, nj)| ni < H && nj < W)
                {
                    if s[ni][nj] != 2 && !cmp.contains(&(ni, nj)) {
                        stack.push((ni, nj))
                    }
                }
            }
        }
        ans = ans.max(cmp.len())
    }
    println!("{}", ans);
}
