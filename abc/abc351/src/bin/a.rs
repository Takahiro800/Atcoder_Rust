#![allow(non_snake_case)]
use std::usize;

use proconio::input;

fn main() {
    input! {
        A: [usize;9],
        B: [usize;8]
    };

    let a = A.iter().sum::<usize>();
    let b = B.iter().sum::<usize>();

    let ans = a - b + 1;
    println!("{}", ans);
}
