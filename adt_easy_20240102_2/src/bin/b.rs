#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let nums: Vec<usize> = S
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let sum: usize = nums.into_iter().sum();
    println!("{}", 45 - sum)
}
