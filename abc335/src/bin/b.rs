#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: i32
    };

    for i in 0..=N {
        for j in 0..=N {
            for k in 0..=N {
                if i >= 0 && j >= 0 && k >= 0 && i + j + k <= N {
                    println!("{} {} {}", i, j, k)
                }
            }
        }
    }
}
