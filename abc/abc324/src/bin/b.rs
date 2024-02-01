#![allow(non_snake_case)]
use std::usize;

use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut ans = false;
    for i in 0..64 {
        for j in 0..64 {
            ans |= 2usize
                .saturating_pow(i)
                .saturating_mul(3usize.saturating_pow(j))
                == N;
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
