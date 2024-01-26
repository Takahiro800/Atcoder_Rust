#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars
    };

    for (i, s) in S.iter().enumerate() {
        if i % 2 == 1 && s == &'1' {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
