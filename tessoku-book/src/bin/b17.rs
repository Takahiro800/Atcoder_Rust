use itertools::Itertools;
use proconio::input;
use std::{cmp, collections::VecDeque};

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut dp: Vec<usize> = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 0;
    dp[2] = abs(h[0], h[1]);

    for i in 3..=n {
        dp[i] = cmp::min(
            dp[i - 2] + abs(h[i - 1], h[i - 3]),
            dp[i - 1] + abs(h[i - 1], h[i - 2]),
        );
    }

    let mut ans: VecDeque<usize> = VecDeque::new();
    let mut current_place = n;

    while current_place > 0 {
        ans.push_front(current_place);

        if current_place == 1 {
            break;
        }

        if dp[current_place]
            == dp[current_place - 1] + abs(h[current_place - 1], h[current_place - 2])
        {
            current_place -= 1;
        } else {
            current_place -= 2;
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}

fn abs(x: usize, y: usize) -> usize {
    if x >= y {
        return x - y;
    }
    return y - x;
}
