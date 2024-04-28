#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        A: [Chars;N],
        B: [Chars;N]
    };

    for (i, j) in iproduct!(0..N, 0..N) {
        if A[i][j] != B[i][j] {
            println!("{} {}", i + 1, j + 1);
            return;
        }
    }
}
