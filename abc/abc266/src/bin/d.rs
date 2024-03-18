#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        TXN: [(usize, usize, isize);N]
    };

    let inf = std::isize::MAX;
    let mut dp = [-inf; 5];
    dp[0] = 0;

    let mut x = 0;

    let lim = TXN.last().unwrap().0;

    for i in 1..=lim {
        let mut next = dp;

        for (next, dp) in next[1..].iter_mut().zip(dp.iter()) {
            *next = std::cmp::max(*next, *dp)
        }
        for (next, dp) in next.iter_mut().zip(dp[1..].iter()) {
            *next = std::cmp::max(*next, *dp)
        }

        dp = next;

        if x < N && TXN[x].0 == i {
            let (_, y, n) = TXN[x];
            dp[y] += n;
            x += 1;
        }
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
