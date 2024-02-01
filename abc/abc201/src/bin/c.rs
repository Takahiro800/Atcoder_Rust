#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        S: Bytes
    };

    let mut ans = 0;
    for mut i in 0..=9999 {
        let mut c = [false; 10];
        for _ in 0..4 {
            c[i % 10] = true;
            i /= 10;
        }

        ans += (0..10).all(|i| match S[i] {
            b'o' => c[i],
            b'x' => !c[i],
            _ => true,
        }) as usize
    }

    println!("{}", ans);
}
