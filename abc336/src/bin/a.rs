#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let o = "o".repeat(N);
    println!("L{}ng", o);
}
