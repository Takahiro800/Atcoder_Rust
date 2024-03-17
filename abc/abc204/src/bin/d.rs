#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        T: [usize; N]
    };

    let mut dp = vec![true];
    for t in T {
        dp.extend(vec![false; t]);

        for i in (t..dp.len()).rev() {
            dp[i] |= dp[i - t];
        }
    }

    let sum = dp.len() - 1;
    let mut ans = sum;

    for (i, &dp) in dp.iter().enumerate() {
        if dp {
            ans = ans.min(i.max(sum - i))
        }
    }
    println!("{}", ans);
}
