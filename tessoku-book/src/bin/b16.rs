use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut dp: Vec<usize> = vec![0; n];
    dp[0] = 0;
    dp[1] = abs(h[0], h[1]);

    for i in 2..n {
        dp[i] += cmp::min(
            dp[i - 2] + abs(h[i], h[i - 2]),
            dp[i - 1] + abs(h[i], h[i - 1]),
        );
    }
    println!("{}", dp[n - 1]);
}

fn abs(x: usize, y: usize) -> usize {
    if x >= y {
        return x - y;
    }
    return y - x;

    // versionの関係で使えなかった
    // return x.abs_diff(y);
}
