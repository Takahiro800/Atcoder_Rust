#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: Chars,
    };

    for w in N.windows(2) {
        if w[0] <= w[1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
