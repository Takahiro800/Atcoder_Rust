#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        A: usize,
        B: usize,
        D: usize,
    };

    let mut p = A;

    while p <= B {
        println!("{}", p);
        p += D;
    }
}
