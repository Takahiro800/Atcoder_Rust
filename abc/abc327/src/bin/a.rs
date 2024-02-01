#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        _N: usize,
        S: Bytes
    };

    let ans = S
        .windows(2)
        .any(|w| (w[0] == b'a' && w[1] == b'b') || (w[0] == b'b' && w[1] == b'a'));

    println!("{}", if ans { "Yes" } else { "No" });
}
