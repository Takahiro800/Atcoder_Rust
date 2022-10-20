use std::i64::MIN;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        W: usize,
        WV: [[i64;2]; N]
    }

    // dpにboolとusizeを混合させることができないので、実現不可能な部分は負の数を入れることにする
    let min: i64 = MIN;

    let mut dp: Vec<Vec<i64>> = vec![vec![min; W + 1]; N + 1];
    dp[0][0] = 0;

    let mut current_max: i64 = 0;

    for i in 1..=N {
        for j in 0..=W {
            // これは、配列のインデックスがマイナスにならないように場合訳している
            if (j as i64) < WV[i - 1][0] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(
                    dp[i - 1][j],
                    dp[i - 1][j - WV[i - 1][0] as usize] + WV[i - 1][1],
                )
            }
            current_max = max(dp[i][j], current_max);
        }
    }
    println!("{}", current_max);
}

fn max(x: i64, y: i64) -> i64 {
    if x >= y {
        return x;
    }
    return y;
}
