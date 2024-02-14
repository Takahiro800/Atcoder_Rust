#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _N: usize,
        S: Chars
    };

    let mut v = vec![];
    let mut ans = vec![];
    for c in S.iter() {
        match c {
            '(' => {
                v.push(ans.len());
                ans.push(c);
            }
            ')' => {
                if let Some(s) = v.pop() {
                    ans.truncate(s);
                } else {
                    ans.push(c);
                }
            }
            _ => ans.push(c),
        }
    }
    println!("{}", ans.iter().join(""));
}
