#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
use superslice::*;

fn main() {
    input! {
         N : usize
    };
    println!("{}", N.trailing_zeros());
}
