#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        A: [[usize;N];N],
        M: usize,
        XY: [(Usize1, Usize1);M],
    };

    let mut ok = vec![vec![true; N]; N];

    for &(x, y) in XY.iter() {
        ok[x][y] = false;
        ok[y][x] = false;
    }

    let mut ans = std::usize::MAX;
    for p in (0..N).permutations(N) {
        if p.windows(2).all(|w| ok[w[0]][w[1]]) {
            let sum = p.iter().enumerate().map(|(i, n)| A[*n][i]).sum::<usize>();
            ans = ans.min(sum);
        }
    }

    if ans == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
