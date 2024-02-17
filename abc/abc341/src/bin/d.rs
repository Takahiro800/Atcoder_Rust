#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    };

    let lcm = lcm(N, M);
    let mut ng = 0;
    let mut ok = 10_usize.pow(18) * 2;

    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        let cnt = mid / N + mid / M - mid / lcm * 2;

        if cnt >= K {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{}", ok)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
