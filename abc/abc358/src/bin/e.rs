#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        K: usize,
        C: [usize;26],
    };

    let M = 998244353;

    let mut v = vec![];
    for (i, &c) in C.iter().enumerate() {
        v.push((i + 'a') as char);
    }

    println!("{}", dp[K]);
}
