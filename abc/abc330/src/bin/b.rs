#![allow(non_snake_case)]
use itertools::*;
use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        R: usize,
        A: [usize; N]
    };

    let mut ans = vec![];

    for a in &A {
        match (&L < a, a < &R) {
            (false, _) => ans.push(&L),
            (true, true) => ans.push(a),
            (true, false) => ans.push(&R),
        }
    }

    println!("{}", ans.iter().join(" "))
}
