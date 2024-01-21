#![allow(non_snake_case)]
use num::pow;
use proconio::input;

fn main() {
    input! {
        B: usize
    };

    for i in 1..18 {
        if pow(i, i) == B {
            println!("{}", i);
            return;
        }

        if pow(i, i) > B {
            println!("-1");
            return;
        }
    }

    println!("-1")
}
