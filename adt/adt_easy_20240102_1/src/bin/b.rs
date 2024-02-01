#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        K: usize
    };

    println!("{}", alphabet_string(K))
}

fn alphabet_string(k: usize) -> String {
    let alphabet: Vec<char> = (b'A'..=b'Z').map(|c| c as char).collect();
    let mut s = String::new();
    for c in alphabet.iter().take(k) {
        s.push(*c);
    }

    s
}
