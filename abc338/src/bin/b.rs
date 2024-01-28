#![allow(non_snake_case)]
use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        S: String
    };

    let ans = most_frequent_char(&S);
    println!("{}", ans);
}

fn most_frequent_char(s: &str) -> char {
    let mut counts = [0; 26];
    for c in s.chars() {
        counts[c as usize - 'a' as usize] += 1;
    }
    let max_count = *counts.iter().max().unwrap();
    (counts.iter().position(|&x| x == max_count).unwrap() as u8 + 'a' as u8) as char
}
