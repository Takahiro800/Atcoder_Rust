#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Bytes};
// use superslice::*;

fn main() {
    input! {
        _N: usize,
        S: Bytes
    };

    let mut set = [false; 3];
    for (i, s) in S.iter().enumerate() {
        let b = s - b'A';
        set[b as usize] = true;

        if set.iter().all(|&bool| bool) {
            println!("{}", i + 1);
            break;
        }
    }
}
