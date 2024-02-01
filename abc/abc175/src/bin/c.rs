#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        X: isize,
        K: usize,
        D: usize,
    };

    let a = X.abs() as usize / D;
    let b = X.abs() as usize % D;

    let ans = match a.cmp(&K) {
        std::cmp::Ordering::Greater => X.abs() as usize - (K * D),
        _ => {
            if a % 2 == K % 2 {
                b
            } else {
                (b).abs_diff(D)
            }
        }
    };

    println!("{}", ans);
}
