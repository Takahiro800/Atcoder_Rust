use proconio::input;
use std::{cmp, collections::VecDeque};

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut A: [usize; n-1],
        B: [usize; n-2]
    }

    let mut C: VecDeque<usize> = VecDeque::from(A);
    C.push_front(0);

    let mut D: VecDeque<usize> = VecDeque::from(B);
    D.push_front(0);
    D.push_front(0);

    let mut dp: Vec<usize> = vec![0; n];
    dp[0] = 0;
    dp[1] = C[1];

    for i in 2..n {
        dp[i] = cmp::min(dp[i - 1] + C[i], dp[i - 2] + D[i]);
    }
    println!("{}", dp[n - 1])
}
