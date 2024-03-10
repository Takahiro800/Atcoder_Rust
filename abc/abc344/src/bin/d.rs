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

    let mut dp = vec![std::usize::MAX; T.len()];
    // let mut dp = vec![0; T.len()];
    for s in S.iter() {
        for s in s.iter() {
            for i in 0..dp.len() {
                let l = i + s.len();
                if l < T.len() {
                    if &T[i..l] == s.as_str() {
                        let a = dp[i];
                        // dp[l] = dp[l].min(a.wrapping_add(1))
                        if dp[l] == std::usize::MAX {
                            dp[l] = 1;
                        } else if a == std::usize::MAX {
                            dp[l] = 1;
                        } else {
                            dp[l] = dp[l].min(a + 1);
                        }
                    }
                }
            }
        }
    }

    // let ans = dp.iter().min().unwrap();
    let ans = dp.last().unwrap();
    println!(
        "{}",
        if *ans == std::usize::MAX {
            -1
        } else {
            *ans as isize
        }
    );
}
