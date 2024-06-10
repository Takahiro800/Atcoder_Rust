#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        AC: [(usize, usize);N]
    };

    let mut indexed_AC: Vec<(usize, (usize, usize))> = AC.iter().cloned().enumerate().collect();
    indexed_AC.sort_by_key(|(_, (_, cost))| cost.clone());

    let mut p = 0;
    indexed_AC.retain(|&(_, (power, _))| {
        if power > p {
            p = power;
            true
        } else {
            false
        }
    });
    indexed_AC.sort_by_key(|(i, (_, _))| i.clone());
    let mut ans = indexed_AC.iter().map(|(i, (_, _))| i.clone() + 1);
    println!("{}", ans.len());
    println!("{}", ans.join(" "));
}
