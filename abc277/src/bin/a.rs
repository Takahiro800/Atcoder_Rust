#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        P: [usize; N]
    };

    // for i in (0..N) {
    for (i, _) in P.iter().enumerate() {
        if P[i] == X {
            println!("{}", i + 1);
            return;
        }
    }
    return;
}
