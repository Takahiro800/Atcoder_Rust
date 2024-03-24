#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        N: usize,
        S: Bytes,
        C: [usize; N]
    };
    let max = usize::MAX;

    let dp = vec![max; 2];
    let mut prev_A = dp.clone();
    let mut prev_B = dp.clone();

    for (i, s) in S.iter().enumerate() {
        let mut A = prev_A.clone();
        let mut B = prev_B.clone();

        if i == 0 {
            if *s == b'0' {
                A[0] = 0;
                B[0] = C[i];
            } else {
                A[0] = C[i];
                B[0] = 0;
            }
        } else {
            if *s == b'0' {
                A[0] = prev_B[0];
                A[1] = prev_A[0].min(prev_B[1]);
                B[0] = prev_A[0].saturating_add(C[i]);
                B[1] = (prev_A[1].saturating_add(C[i])).min(prev_B[0].saturating_add(C[i]));
            } else {
                A[0] = prev_B[0].saturating_add(C[i]);
                A[1] = (prev_B[1].saturating_add(C[i])).min(prev_A[0].saturating_add(C[i]));
                B[0] = prev_A[0];
                B[1] = prev_B[0].min(prev_A[1]);
            }
        }

        prev_A = A;
        prev_B = B;
    }

    let ans = prev_A[1].min(prev_B[1]);
    println!("{}", ans);
}
