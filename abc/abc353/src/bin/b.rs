#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize;N]
    };

    let mut count = 0;
    let mut empty = K;

    let mut i = 0;

    while i < A.len() {
        if A[i] <= empty {
            empty -= A[i];
            i += 1;
        } else {
            count += 1;
            empty = K;
        }
    }

    if empty < K {
        count += 1;
    }
    println!("{}", count);
}
