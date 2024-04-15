#![allow(non_snake_case)]
use proconio::{input, marker::Chars};
use regex::Regex;

fn main() {
    input! {
        S: String,
        T: Chars
    };

    let S = S.to_uppercase();
    let ans = (T[2] == 'X'
        && Regex::new(&format!(".*{}.*{}.*", T[0], T[1]))
            .unwrap()
            .is_match(&S))
        || Regex::new(&format!(".*{}.*{}.*{}.*", T[0], T[1], T[2]))
            .unwrap()
            .is_match(&S);
    println!("{}", if ans { "Yes" } else { "No" });
}
