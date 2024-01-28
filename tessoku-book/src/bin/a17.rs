use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N-1],
        B: [usize; N-2]
    }

    let mut dp = vec![0; N];
    let mut prev = vec![0; N];
    dp[1] = A[0];

    for i in 2..N {
        let w1 = dp[i - 1] + A[i - 1];
        let w2 = dp[i - 2] + B[i - 2];
        dp[i] = w1.min(w2);
        prev[i] = if w1 < w2 { i - 1 } else { i - 2 };
    }

    let mut cur = N - 1;
    let mut path = Vec::new();
    while cur != 0 {
        path.push(cur);
        cur = prev[cur];
    }
    path.push(cur);
    path.reverse();

    println!("{}", path.len());
    println!("{}", path.iter().map(|i| i + 1).join(" "))
}
