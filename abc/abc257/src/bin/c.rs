#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};
use std::collections::BTreeMap;

fn main() {
    input! {
        N: usize,
        S: Bytes,
        mut W: [usize;N]
    };

    let mut map = BTreeMap::new();

    for (c, w) in S.iter().zip(W.iter()) {
        let count = map.entry(*w).or_insert([0; 2]);
        count[(*c - b'0') as usize] += 1;
    }

    let mut cnt = map.values().map(|p| p[1]).sum::<usize>();
    let mut ans = cnt;
    for (_, c) in map {
        cnt -= c[1];
        cnt += c[0];
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}
