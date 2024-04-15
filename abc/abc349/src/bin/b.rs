#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! { S: String }

    let mut hist = vec![0; 26];
    for c in S.chars() {
        hist[(c as u8 - b'a') as usize] += 1;
    }

    let s = hist;
    let mut hist = HashMap::new();

    for &x in s.iter() {
        *hist.entry(x).or_insert(0) += 1;
    }
    let ans = hist.iter().all(|(&k, &v)| k == 0 || matches!(v, 0 | 2));
    println!("{}", if ans { "Yes" } else { "No" });
}
