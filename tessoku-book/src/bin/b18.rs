use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        s: usize,
        A: [usize; n]
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];
    let mut total_count = 0;

    dp[0][0] = true;
    for i in 1..=n {
        for j in 0..=s {
            if dp[i - 1][j] == false {
                continue;
            }

            dp[i][j] = true;

            if j + A[i - 1] == s {
                dp[i][j + A[i - 1]] = true;
                total_count = i;
                break;
            }

            if j + A[i - 1] < s {
                dp[i][j + A[i - 1]] = true;
            }
        }
    }

    if dp[n][s] == false {
        println!("{}", -1);
        return;
    }

    let mut r = s;
    let mut ans: VecDeque<usize> = VecDeque::new();

    while total_count > 0 {
        if r >= A[total_count - 1] && dp[total_count - 1][r - A[total_count - 1]] == true {
            ans.push_front(total_count);
            r -= A[total_count - 1];
        }
        total_count -= 1;
    }
    println!("{}", ans.iter().len());
    println!("{}", ans.iter().join(" "));
}
