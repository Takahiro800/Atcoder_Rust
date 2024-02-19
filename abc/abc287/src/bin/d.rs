#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut S: Bytes,
        mut T: Bytes,
    };

    let len = T.len() + 1;
    let mut prefix = vec![false; len];
    let mut suffix = vec![false; len];

    prefix[0] = true;
    suffix[0] = true;

    for i in 1..=T.len() {
        if prefix[i - 1] {
            if S[i - 1] == b'?' || T[i - 1] == b'?' || S[i - 1] == T[i - 1] {
                prefix[i] = true;
            }
        }

        if suffix[i - 1] {
            if S[S.len() - i] == b'?' || T[T.len() - i] == b'?' || S[S.len() - i] == T[T.len() - i]
            {
                suffix[i] = true;
            }
        }
    }
    suffix.reverse();

    for (&p, &s) in suffix.iter().zip(prefix.iter()) {
        let ans = p && s;
        println!("{}", if ans { "Yes" } else { "No" });
    }
}
