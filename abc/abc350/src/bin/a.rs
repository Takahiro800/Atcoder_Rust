#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let s = &S[3..];
    let num: usize = s.parse().unwrap();

    if num < 350 && num != 316 && num > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
