use std::cmp;
use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        mut A: [usize; n-1],
        B: [usize; n-2]
    }

    let mut C: VecDeque<usize> = VecDeque::from(A);
    C.push_front(0);
    C.push_front(0);

    let mut D: VecDeque<usize> = VecDeque::from(B);
    D.push_front(0);
    D.push_front(0);
    D.push_front(0);

    // 動的計画法
    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = C[2];

    for i in 3..=n {
        dp[i] = cmp::min(dp[i - 1] + C[i], dp[i - 2] + D[i]);
    }

    let mut ans: VecDeque<usize> = VecDeque::new();

    // 復元する
    let mut current_place: usize = n;

    while current_place > 0 {
        ans.push_front(current_place);

        if current_place == 1 {
            break;
        }

        if dp[current_place] == dp[current_place - 1] + C[current_place] {
            current_place -= 1;
        } else {
            current_place -= 2;
        }
    }

    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
