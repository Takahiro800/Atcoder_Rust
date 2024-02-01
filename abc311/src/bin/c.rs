#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N: usize,
        A: [Usize1;N]
    };

    let mut used = vec![false; N];
    let mut p = 0;
    while !used[p] {
        used[p] = true;
        p = A[p];
    }

    let mut ans = vec![p];
    let mut x = A[p];

    while x != p {
        ans.push(x);
        x = A[x];
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().map(|a| *a + 1).join(" "));
}
