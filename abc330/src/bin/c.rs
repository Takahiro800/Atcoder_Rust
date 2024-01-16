#![allow(non_snake_case)]
use std::f64;

// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        D: f64
    };

    let d = D.sqrt().ceil() as usize;
    let mut ans = D as usize;

    for i in 0..=d {
        let diff = if (i * i) as f64 >= D {
            diff(i, 0, D as usize)
        } else {
            let f = (D - (i * i) as f64).sqrt().floor() as usize;
            let c = (D - (i * i) as f64).sqrt().ceil() as usize;

            diff(i, f, D as usize).min(diff(i, c, D as usize))
        };

        if diff < ans {
            ans = diff;
        }
    }

    println!("{}", ans)
}

fn diff(x: usize, y: usize, D: usize) -> usize {
    D.abs_diff(x * x + y * y)
}
