#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String,
        T: String
    };

    if T.len() > S.len() {
        println!("No");
        return;
    }

    for i in 0..=(S.len() - T.len()) {
        let sub = &S[i..i + T.len()];
        if sub == T {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
