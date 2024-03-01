#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut S: Bytes
    };

    let f = S[0];

    for (i, s) in S.iter().enumerate() {
        if s != &f {
            if i >= 2 {
                println!("{}", i + 1);
                return;
            } else {
                if f == S[2] {
                    println!("2");
                    return;
                } else {
                    println!("1");
                    return;
                }
            }
        }
    }
}
