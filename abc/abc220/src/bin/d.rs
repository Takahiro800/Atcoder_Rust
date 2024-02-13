#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };

    const MOD: usize = 998_244_353;
    let mut dp = [0; 10];
    dp[A[0]] += 1;

    for &a in A[1..].iter() {
        let mut next = [0; 10];
        for (i, p) in dp.iter().enumerate() {
            next[(i + a) % 10] += p;
            next[i * a % 10] += p;
        }

        dp = next;
        dp.iter_mut().for_each(|p| *p %= MOD)
    }

    for ans in dp.iter() {
        println!("{}", ans);
    }
}
