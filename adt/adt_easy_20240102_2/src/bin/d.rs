#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String,
        T: String
    };

    if T.starts_with(&S) {
        println!("Yes")
    } else {
        println!("No")
    }
}
