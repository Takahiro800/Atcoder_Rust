#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        pages: [(usize, usize, usize, usize);N]
    };

    let h = 1001;
    let w = 1001;

    let mut grid = vec![vec![0_isize; w]; h];
    for &(a, b, c, d) in pages.iter() {
        grid[b][a] += 1;
        grid[d][c] += 1;
        grid[d][a] -= 1;
        grid[b][c] -= 1;
    }

    let mut cnt = vec![0; N + 1];
    for i in 0..h {
        for j in 0..w {
            if i > 0 {
                grid[i][j] += grid[i - 1][j];
            }
            if j > 0 {
                grid[i][j] += grid[i][j - 1];
            }

            if i > 0 && j > 0 {
                grid[i][j] -= grid[i - 1][j - 1];
            }
            cnt[grid[i][j] as usize] += 1;
        }
    }
    for &ans in cnt[1..].iter() {
        println!("{}", ans);
    }
}
