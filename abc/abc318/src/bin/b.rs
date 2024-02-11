#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        N: usize,
    };

    let mut D = vec![vec![false; 100]; 100];

    for _ in 0..N {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize
        }

        for x in c..d {
            for y in a..b {
                D[y][x] = true;
            }
        }
    }

    let ans = D.iter().flatten().filter(|&p| *p).count();
    println!("{}", ans);
}
