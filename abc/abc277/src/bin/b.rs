#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut S: [String; N]
    };
    let mut T = S.clone();

    let HDCS: Vec<char> = "HDCS".chars().collect();
    let num: Vec<char> = "A23456789TJQK".chars().collect();
    for s in S {
        let ch1 = s.chars().next().unwrap();
        let ch2 = s.chars().nth(1).unwrap();

        // if !["H", "D", "C", "S"].contains(ch1) {
        if !HDCS.contains(&ch1) {
            println!("No");
            return;
        }

        if !num.contains(&ch2) {
            println!("No");
            return;
        }
    }

    T.sort();
    T.dedup();
    if T.len() != N {
        println!("No");
        return;
    }
    println!("Yes");
}
