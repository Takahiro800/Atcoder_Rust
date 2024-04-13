#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        S: Chars,
        mut T: Chars
    };

    let S: Vec<char> = S
        .into_iter()
        .map(|c| c.to_uppercase().next().unwrap())
        .collect();

    if *T.last().unwrap() == 'X' {
        T.pop();
    }

    let mut p: isize = -1;
    let mut count = 0;
    for t in T.iter() {
        for (i, s) in S.iter().enumerate() {
            if t == s && i as isize > p {
                p = i as isize;
                count += 1;
                break;
            }
        }
    }

    let ans = count == T.len();
    println!("{}", if ans { "Yes" } else { "No" });
}
