#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut ans = vec![];
    for i in 1..=N {
        if i % 3 == 0 {
            ans.push('x')
        } else {
            ans.push('o')
        }
    }
    println!("{}", ans.iter().join(""));
}
