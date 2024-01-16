#![allow(non_snake_case)]
use std::f64;

// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        D: usize
    };

    let d = (D as f64).sqrt().ceil() as usize;
    let mut ans = D;

    for i in 0..=d {
        let diff = if i * i >= D {
            diff(i, 0, D)
        } else {
            let f = ((D - (i * i)) as f64).sqrt().floor() as usize;
            let c = ((D - (i * i)) as f64).sqrt().ceil() as usize;

            diff(i, f, D).min(diff(i, c, D))
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
