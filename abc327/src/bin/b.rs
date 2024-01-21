#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        B: usize
    };

    for i in 1usize..18 {
        if i.saturating_pow(i as u32) == B {
            println!("{}", i);
            return;
        }
    }

    println!("-1")
}
