#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        AB: [(usize, usize); N],
    };
    let mut nums = AB.iter().flat_map(|&(a, b)| [a - b, b]).collect_vec();
    nums.sort();
    nums.reverse();

    let ans: usize = nums.iter().take(K).sum();
    println!("{}", ans);
}
