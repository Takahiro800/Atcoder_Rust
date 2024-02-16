#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize;N]
    };

    let mut cnt = vec![0; N];
    for (i, p) in P.iter().enumerate() {
        let s = (p + N - i - 1) % N;
        for i in 0..3 {
            cnt[(s + i) % N] += 1;
        }
    }

    println!("{}", cnt.iter().max().unwrap());
}
