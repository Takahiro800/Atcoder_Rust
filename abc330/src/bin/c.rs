#![allow(non_snake_case)]

// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        D: usize
    };

    let mut y = 0;
    let mut ans = !0usize;

    for x in (0..=2_000_000).rev() {
        while x * x + (y + 1) * (y + 1) <= D {
            y += 1;
        }

        ans = ans.min(D.abs_diff(x * x + y * y));
        ans = ans.min(D.abs_diff(x * x + (y + 1) * (y + 1)));
    }

    println!("{}", ans)
}
