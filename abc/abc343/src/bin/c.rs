#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut a = vec![];
    for i in 1..=N {
        if i * i * i > N {
            break;
        }
        a.push(i * i * i);
    }
    a.reverse();

    for a in a.iter() {
        let s = a.to_string();
        if s == s.chars().rev().collect::<String>() {
            println!("{}", s);
            return;
        }
    }
}
