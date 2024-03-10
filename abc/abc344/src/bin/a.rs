#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let parts: Vec<&str> = S.split('|').collect();
    println!("{}{}", parts[0], parts[2]);
}
