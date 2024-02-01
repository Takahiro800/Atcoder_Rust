#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: i32
    };

    for x in 0..=N {
        for y in 0..=(N - x) {
            for z in 0..=(N - x - y) {
                println!("{} {} {}", x, y, z)
            }
        }
    }
}
