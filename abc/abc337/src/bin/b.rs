#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut S: Chars
    };

    S.dedup();

    let ans = S.len() <= 3 && S.windows(2).all(|w| w[0] < w[1]);
    println!("{}", if ans { "Yes" } else { "No" });
}
