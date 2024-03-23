#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };
    let mut ans = vec![];

    for w in A.windows(2) {
        ans.push(w[0] * w[1])
    }
    println!("{}", ans.iter().join(" "));
}
