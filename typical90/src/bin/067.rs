#![allow(non_snake_case)]
use proconio::input;
use std::char;

fn main() {
    input! {
        mut N: String,
        K: usize,
    };

    for _ in 0..K {
        let mut x = N
            .chars()
            .map(|c| (c as u8 - b'0') as u64)
            .fold(0, |acc, x| acc * 8 + x);
        let mut t = Vec::new();

        while x != 0 {
            let r = x % 9;
            t.push(if r == 8 {
                '5'
            } else {
                (b'0' + r as u8) as char
            });
            x /= 9;
        }
        N = t.iter().rev().collect();
    }

    if N.is_empty() {
        println!("0");
    } else {
        println!("{}", &N);
    }
}
