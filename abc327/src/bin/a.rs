#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _N: usize,
        S: Chars
    };

    for i in 0..S.len() - 1 {
        if (S[i] == 'a' && S[i + 1] == 'b') || (S[i] == 'b' && S[i + 1] == 'a') {
            println!("Yes");
            return;
        }
    }
    println!("No")
}
