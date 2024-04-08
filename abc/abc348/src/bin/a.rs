#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    println!("{}", "oox".chars().cycle().take(N).join(""));
}
