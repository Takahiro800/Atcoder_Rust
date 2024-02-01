#![allow(non_snake_case)]
use std::{char, collections::HashSet};

use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String,
    };

    let chars: Vec<char> = S.chars().collect();

    let unique_chars: HashSet<_> = chars.clone().into_iter().collect();
    if unique_chars.len() == 1 {
        println!("{}", -1);
        return;
    }

    let mut counter = 0;
    let mut ans = 0;
    for c in chars {
        if c == '-' {
            if counter > ans {
                ans = counter;
            }
            counter = 0;
        } else {
            counter += 1;
        }
    }

    if counter > ans {
        ans = counter;
    }
    println!("{}", ans)
}
