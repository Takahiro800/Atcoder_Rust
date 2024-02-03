#![allow(non_snake_case)]
use itertools::Itertools;
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        H: usize,
        W: usize,
        N: usize
    };

    let mut grid = vec![vec!['.'; W]; H];
    let mut i = 0;
    let mut j = 0;
    let dir = [(H - 1, 0), (0, 1), (1, 0), (0, W - 1)];
    let mut cd = 0;

    for _ in 0..N {
        match grid[i][j] {
            '.' => {
                grid[i][j] = '#';
                cd = (cd + 1) % 4
            }
            '#' => {
                grid[i][j] = '.';
                cd = (cd + 3) % 4
            }
            _ => unreachable!(),
        }

        i = (i + dir[cd].0) % H;
        j = (j + dir[cd].1) % W;
    }

    for line in grid {
        println!("{}", line.iter().join(""))
    }
}
