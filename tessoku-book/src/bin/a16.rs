use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize;N-1],
        B: [usize; N-2]
    }

    let mut dp = vec![0; N];
    dp[1] = A[0];

    for i in 2..N {
        dp[i] = (dp[i - 1] + A[i - 1]).min(dp[i - 2] + B[i - 2]);
    }
    println!("{}", dp[N - 1]);
}
