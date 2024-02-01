#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize
    };

    let mut r = vec![0; N];
    let mut b = vec![0; N];

    b[0] = 1;

    for i in 1..N {
        b[i] = r[i - 1] + Y * b[i - 1];
        r[i] = r[i - 1] + X * b[i];
    }

    println!("{}", r[N - 1]);
}
