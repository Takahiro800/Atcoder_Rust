#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String
    };

    let chars: Vec<char> = S.chars().collect();
    let char_set: HashSet<char> = chars.clone().into_iter().collect();

    if char_set.len() == 1 {
        println!("{}", -1);
        return;
    }

    let mut count = 0;
    let mut ans = 0;
    for c in chars {
        if c == '-' {
            if count > ans {
                ans = count;
            }
            count = 0;
        } else {
            count += 1;
        }
    }

    if count > ans {
        ans = count;
    }
    println!("{}", ans)
}
