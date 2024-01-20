#![allow(non_snake_case)]
use itertools::*;
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [isize;N]
    };

    let mut stock = vec![0; N + 1];
    let mut ans = vec![];

    for i in 0..A.len() {
        if A[i] == -1 {
            ans.push(i + 1);
        } else {
            stock[(A[i]) as usize] = i + 1;
        }
    }

    for i in 1..A.len() {
        let target = ans[i - 1];
        ans.push(stock[target]);
    }

    println!("{}", ans.iter().join(" "))
}
