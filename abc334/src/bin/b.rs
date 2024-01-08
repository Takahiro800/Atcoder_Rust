#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        A: isize,
        M: isize,
        L: isize,
        R: isize,
    };

    let max = floor(R - A, M);
    let min = floor(L - A + M - 1, M);
    let ans = max - min + 1;

    println!("{}", ans)
}

fn floor(a: isize, b: isize) -> isize {
    let r = (a % b + b) % b;
    (a - r) / b
}

#[test]
fn test_floor() {
    assert_eq!(floor(-3, 2), -2);
    assert_eq!(floor(-1 - 5 + 3 + 1, 2), -2);
}
