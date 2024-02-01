#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: [String;3],
        T: String
    };

    let T2: Vec<_> = T
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize - 1)
        .collect();
    let ans = T2
        .iter()
        .map(|t| S[*t].clone())
        .collect::<Vec<_>>()
        .join("");
    println!("{}", ans)
}
