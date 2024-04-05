#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N:usize,
        K: usize,
         A: [usize; N]
    };

    let a = A.iter().filter(|x| *x % K == 0).collect::<Vec<_>>();
    let ans: Vec<usize> = a.iter().map(|a| *a / K).sorted().collect();
    println!("{}", ans.iter().join(" "));
}
