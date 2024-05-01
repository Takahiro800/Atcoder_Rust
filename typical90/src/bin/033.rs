#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
    };

    let ans = if H == 1 || W == 1 {
        H * W
    } else {
        ((H + 1) / 2) * ((W + 1) / 2)
    };

    println!("{}", ans);
}
