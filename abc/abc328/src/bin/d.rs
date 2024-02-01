#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        S: Chars
    };

    let mut ans = vec![];
    for s in S {
        ans.push(s);
        if ans.ends_with(&['A', 'B', 'C']) {
            ans.pop();
            ans.pop();
            ans.pop();
        }
    }
    println!("{}", ans.iter().join(""));
}
