#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        K: usize,
        G: usize,
        M: usize
    };

    let mut g = 0;
    let mut m = 0;

    for _ in 0..K {
        if g == G {
            g = 0;
        } else if m == 0 {
            m = M;
        } else {
            let r = std::cmp::min(m, G - g);
            g += r;
            m -= r;
        }
    }
    println!("{} {}", g, m)
}
