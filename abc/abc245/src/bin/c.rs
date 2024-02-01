#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize;N],
        B: [usize;N],
    };

    let mut dp: Vec<bool> = vec![false; N];
    let mut ep: Vec<bool> = vec![false; N];

    dp[0] = true;
    ep[0] = true;

    for i in 1..N {
        if dp[i - 1] {
            if A[i - 1].abs_diff(A[i]) <= K {
                dp[i] = true
            }
            if A[i - 1].abs_diff(B[i]) <= K {
                ep[i] = true
            }
        }

        if ep[i - 1] {
            if B[i - 1].abs_diff(A[i]) <= K {
                dp[i] = true
            }
            if B[i - 1].abs_diff(B[i]) <= K {
                ep[i] = true
            }
        }
    }

    println!("{}", if dp[N - 1] || ep[N - 1] { "Yes" } else { "No" });
}
