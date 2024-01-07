#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        A: isize,
        M: isize,
        L: isize,
        R: isize,
    };

    let left = L - A;
    let right = R - A;

    let l = floor(left - 1, M);
    let r = floor(right, M);

    let ans = r - l;
    println!("{}", ans)
}

fn floor(a: isize, b: isize) -> isize {
    let r = (a % b + b) % b;
    println!("{}", r);
    (a - r) / b
}

#[test]
fn test_floor() {
    assert_eq!(floor(-3, 2), -2)
}
