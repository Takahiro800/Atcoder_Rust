#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n: u8
    };

    let mut s = String::new();

    for _ in 0..n {
        // s.push(n as char);
        s.push_str(&n.to_string());
    }

    println!("{}", s)
}
