#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut count = vec![0; N + 1];
    for a in 1..=N {
        for b in 1..=(N / a) {
            count[a * b] += 1;
        }
    }

    let mut ans = 0;
    for i in 0..N {
        ans += count[i] * count[N - i]
    }
    println!("{}", ans);
}
