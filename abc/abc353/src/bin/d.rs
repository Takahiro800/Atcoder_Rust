#![allow(non_snake_case)]
use std::u128;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [u128;N]
    };

    let MOD = 998244353;

    let pows = A
        .iter()
        .map(|&a| 10_u128.pow(a.to_string().len() as u32))
        .collect_vec();
    let cum_sum: Vec<_> = pows
        .iter()
        .scan(0, |sum, &v| {
            *sum += v;
            Some(*sum)
        })
        .collect_vec();

    let mut ans = 0;
    let last = cum_sum.last().unwrap_or(&0);

    for (i, (a, b)) in A.iter().zip(cum_sum.iter()).enumerate() {
        ans += a * (last - b + i as u128) % MOD;
    }

    println!("{}", ans % MOD);
}
