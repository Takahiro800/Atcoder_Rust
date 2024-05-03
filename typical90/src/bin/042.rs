#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        K: usize
    };

    if K % 9 != 0 {
        println!("{}", 0);
        return;
    }

    let mut dp = vec![0; 9];
    dp[8] = 1;

    for _ in 0..K {
        let s = dp.iter().sum::<usize>() % 1_000_000_007;
        dp.rotate_left(1);
        dp[8] = s;
    }

    let ans = dp[8];
    println!("{}", ans);
}
