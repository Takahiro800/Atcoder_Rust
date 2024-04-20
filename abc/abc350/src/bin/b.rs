#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
        T: [Usize1;Q]
    };

    let mut ans = vec![true; N];

    for &t in T.iter() {
        ans[t] = !ans[t];
    }

    let ans = ans.iter().filter(|&a| *a).count();
    println!("{}", ans);
}
