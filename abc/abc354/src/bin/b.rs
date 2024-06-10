#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut SC: [(String, usize);N],
    };

    let sum: usize = SC.iter().map(|(_, c)| c).sum();
    SC.sort_by_key(|(s, _)| s.clone());

    let n = sum % N;
    let ans = &SC[n].0;
    println!("{}", ans);
}
