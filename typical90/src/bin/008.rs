#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _N: usize,
        S: Chars,
    };

    let MOD = 10_usize.pow(9) + 7;

    let mut v = vec![0; "atcoder".len()];
    let atcoder: Vec<char> = "atcoder".chars().collect();

    for &s in S.iter() {
        if let Some(i) = atcoder.iter().position(|&c| c == s) {
            if i == 0 {
                v[i] += 1;
            } else {
                v[i] += v[i - 1];
                v[i] %= MOD
            }
        }
    }

    let ans = v.last().unwrap();
    println!("{}", ans % MOD);
}
