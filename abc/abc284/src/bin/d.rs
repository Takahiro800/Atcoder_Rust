#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        T: usize,
        N: [usize;T]
    };

    for n in N {
        let (p, q) = (2..)
            .find(|p| n % p == 0)
            .map(|p| {
                if n % (p * p) == 0 {
                    (p, n / p / p)
                } else {
                    (kth_root(n / p, 2), p)
                }
            })
            .unwrap();

        println!("{} {}", p, q);
    }
}

/// floor(a ^ (1 / k)) 自然数のk乗根を返す
fn kth_root(a: usize, k: usize) -> usize {
    assert!(k > 0);
    if a == 0 {
        return 0;
    }
    if k >= 64 {
        return 1;
    }

    if k == 1 {
        return a;
    }

    let valid = |x: usize| -> bool {
        let mut t = x;
        for _ in 1..k {
            let (val, is_overflow) = t.overflowing_mul(x);

            if is_overflow || val > a {
                return false;
            }
            t = val;
        }
        true
    };

    let mut ok = 1;
    let mut ng = 2;

    while valid(ng) {
        ok = ng;
        ng *= 2;
    }
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        if valid(mid) {
            ok = mid;
        } else {
            ng = mid
        }
    }
    ok
}
