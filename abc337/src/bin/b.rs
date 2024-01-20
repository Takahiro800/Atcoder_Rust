#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut S: Chars
    };

    S.dedup();

    match &S[..] {
        ['A'] | ['B'] | ['C'] => println!("Yes"),
        ['A', 'B'] | ['B', 'C'] | ['A', 'C'] => println!("Yes"),
        ['A', 'B', 'C'] => println!("Yes"),
        _ => println!("No"),
    }
}
