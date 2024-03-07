#![allow(non_snake_case)]
use itertools::*;
use proconio::input;

fn main() {
    input! {
        N: usize,
        M:usize,
        mut A: [isize;N+1],
        mut C: [isize; N+M+1]
    };

    A.reverse();
    C.reverse();
    let mut ans = vec![];

    for i in 0..=M {
        let q = C[i] / A[0];
        ans.push(q);

        for (c, a) in C[i..].iter_mut().zip(A.iter()) {
            *c -= a * q;
        }
    }
    ans.reverse();
    println!("{}", ans.iter().join(" "));
}
