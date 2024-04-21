#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        Q: usize,
        B: [usize;Q]
    };

    A.sort();

    for &b in B.iter() {
        let mut p = A.lower_bound(&b);
        if p == N {
            p -= 1;
        }
        let mut diff = A[p].abs_diff(b);

        if p > 0 {
            diff = diff.min(A[p - 1].abs_diff(b));
        }
        println!("{}", diff);
    }
}
