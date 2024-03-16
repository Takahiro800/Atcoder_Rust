#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        S: Bytes
    };
    let len = S.len();

    if S[0] == b'<' && S.last().unwrap() == &b'>' {
        for s in S[1..len - 1].iter() {
            if s != &b'=' {
                println!("No");
                return;
            }
        }
    } else {
        println!("No");
        return;
    }
    println!("Yes")
}
