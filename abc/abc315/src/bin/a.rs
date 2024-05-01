#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
};

fn main() {
    input! {
        mut S: Chars
    };

    let set = ['a', 'i', 'u', 'e', 'o'];
    let mut ans = S.iter().filter(|s| !set.contains(&s));

    println!("{}", ans.join(""));
}
