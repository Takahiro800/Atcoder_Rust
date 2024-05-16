#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [Usize1; M],
    }

    let mut ans = vec![M; N];
    for &a in A.iter() {
        ans[a] -= 1;
    }

    for a in ans.iter() {
        println!("{}", a);
    }
}
