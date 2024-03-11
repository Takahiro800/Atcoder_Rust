#![allow(non_snake_case)]
use std::isize;

use proconio::input;

fn main() {
    input! {
        T: String,
        N: usize,
    };
    let mut S = vec![];

    for _ in 0..N {
        input! {
            a: usize,
            s: [String;a]
        }
        S.push(s);
    }

    // dp[0] = 0としたい
    let dp = vec![std::usize::MAX; T.len() + 1];
    let mut prev = dp;
    for s in S.iter() {
        let mut cur = prev.clone();

        for s in s.iter() {
            for i in 0..prev.len() {
                let l = i + s.len();

                if l <= T.len() {
                    if &T[i..l] == s.as_str() {
                        let a = prev[i];
                        if i == 0 {
                            cur[l] = prev[l].min(1);
                        } else if a < std::usize::MAX {
                            cur[l] = prev[l].min(a + 1);
                        }
                    }
                }
            }
        }
        prev = cur;
    }

    let ans = prev.last().unwrap();
    println!(
        "{}",
        if *ans == std::usize::MAX {
            -1
        } else {
            *ans as isize
        }
    );
}
