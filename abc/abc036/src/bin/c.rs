#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };

    let B = A.clone();
    A.sort();
    A.dedup();

    let mut ans = vec![];

    for b in B {
        let a = A.binary_search(&b).unwrap();
        ans.push(a)
    }

    println!("{}", ans.iter().join(" "));
}
