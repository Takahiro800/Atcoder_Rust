#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        X: isize,
        Y: isize
    };

    if Y - X < 3 && Y - X > -4 {
        println!("Yes")
    } else {
        println!("No")
    }
}
