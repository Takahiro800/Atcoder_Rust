#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        P: [usize; N]
    };

    for (i, p) in P.iter().enumerate() {
        if *p == X {
            println!("{}", i + 1);
            return;
        }
    }
    return;
}
