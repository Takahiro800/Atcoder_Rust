#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut S:Bytes
    };

    let a = "atcoder".bytes().collect::<Vec<_>>();
    let mut ans = 0;

    for (i, a) in a.iter().enumerate() {
        let mut x = i + S[i..].iter().position(|s| s == a).unwrap();
        while x > i {
            ans += 1;
            S.swap(x - 1, x);
            x -= 1;
        }
    }
    println!("{}", ans);
}
