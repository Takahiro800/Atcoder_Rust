#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        H: usize,
        X: usize,
        P: [usize; N]
    };

    let diff = X - H;

    for (i, &p) in P.iter().enumerate() {
        if p >= diff {
            println!("{}", i + 1);
            return;
        }
    }
}
