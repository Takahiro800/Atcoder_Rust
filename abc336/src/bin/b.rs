#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
         N : usize
    };
    println!("{}", N.trailing_zeros());
}
