#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        X: usize,
        Y: usize,
        Z: usize,
        S: Bytes
    };

    let mut on = vec![0; S.len() + 1];
    let mut off = vec![0; S.len() + 1];

    on[0] = Z;

    for i in 0..S.len() {
        match S[i] {
            b'a' => {
                off[i + 1] = (on[i] + Z + X).min(off[i] + X);
                on[i + 1] = (on[i] + Y).min(off[i] + Z + Y);
            }
            b'A' => {
                on[i + 1] = (on[i] + X).min(off[i] + Z + X);
                off[i + 1] = (on[i] + Z + Y).min(off[i] + Y);
            }
            _ => unreachable!(),
        }
    }
    let ans = on[S.len()].min(off[S.len()]);
    println!("{}", ans);
}
