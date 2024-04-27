#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        S: String
    };

    if let Some(pos) = S.rfind('.') {
        println!("{}", &S[pos + 1..]);
    }
}
