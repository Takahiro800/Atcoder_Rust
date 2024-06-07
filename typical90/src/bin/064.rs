#![allow(non_snake_case)]
use std::isize;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [isize; N],
    };

    let mut d = A.windows(2).map(|a| a[1] - a[0]).collect_vec();
    let mut sum = d.iter().map(|d| d.abs()).sum::<isize>();

    for _ in 0..Q {
        input! {
            L: Usize1,
            R: Usize1,
            V: isize,
        }

        if L > 0 {
            sum -= d[L - 1].abs();
            d[L - 1] += V;
            sum += d[L - 1].abs();
        }

        if R < N - 1 {
            sum -= d[R].abs();
            d[R] -= V;
            sum += d[R].abs();
        }

        println!("{}", sum);
    }
}
