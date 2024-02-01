#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        S: Bytes,
        T: Bytes,
    };

    let a = run_length_encoding(S);
    let b = run_length_encoding(T);

    let mut ans = a.len() == b.len();

    for (s, t) in a.into_iter().zip(b) {
        ans &= s.0 == t.0 && { s.1 == t.1 || (s.1 > 1 && s.1 <= t.1) };
    }

    println!("{}", if ans { "Yes" } else { "No" });
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
