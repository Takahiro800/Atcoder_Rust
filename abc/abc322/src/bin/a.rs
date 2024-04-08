#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _N: usize,
        S: Chars
    };

    let mut ans = -1;
    for (i, w) in S.windows(3).enumerate() {
        if w == ['A', 'B', 'C'] {
            ans = i as isize + 1;
            break;
        }
    }

    println!("{}", ans);
}
