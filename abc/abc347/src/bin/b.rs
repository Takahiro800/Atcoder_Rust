#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars
    };

    let mut set = HashSet::new();
    let len = S.len();
    for i in 1..=len {
        S.windows(i).for_each(|c| {
            let chunk_str = c.iter().collect::<String>();
            set.insert(chunk_str);
        });
    }

    let ans = set.len();
    println!("{}", ans);
}
