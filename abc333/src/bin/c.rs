#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let L = 12;

    let r: Vec<usize> = (0..L).map(|i| "1".repeat(i + 1).parse().unwrap()).collect();
    let mut s: Vec<usize> = vec![];

    // 同じ値でも良いので (i+1), (j+1)
    for i in 0..L {
        for j in 0..(i + 1) {
            for k in 0..(j + 1) {
                s.push(r[i] + r[j] + r[k]);
            }
        }
    }
    println!("{}", s[N - 1])
}
