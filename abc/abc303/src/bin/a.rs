#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Bytes};
// use superslice::*;

fn main() {
    input! {
        _N: usize,
        S: Bytes,
        T: Bytes
    };

    let mut ans = true;

    for (&s, &t) in S.iter().zip(&T) {
        let mut temp = s == t;
        for &(a, b) in [(b'1', b'l'), (b'0', b'o')].iter() {
            temp |= (s, t) == (a, b) || (s, t) == (b, a);
        }
        ans &= temp;
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
