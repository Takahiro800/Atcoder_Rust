#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};
use std::{collections::HashSet, usize};

fn main() {
    input! {
        N: usize,
        S: [Bytes;N]
    };
    let mut ans = !0_usize;

    for c in "0123456789".bytes() {
        let a = S
            .iter()
            .map(|s| s.iter().position(|s| *s == c))
            .collect::<Vec<_>>();

        let mut set = HashSet::new();

        for mut a in a.into_iter().flatten() {
            while !set.insert(a) {
                a += 10;
            }
        }
        ans = ans.min(*set.iter().max().unwrap());
    }
    println!("{}", ans);
}
