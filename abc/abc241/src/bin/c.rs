#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        N: usize,
       mut S: [Bytes;N]
    };

    let mut ans = false;
    'outer: for _ in 0..4 {
        for s in S.iter() {
            for s in s.windows(6) {
                if s.iter().filter(|s| **s == b'#').count() >= 4 {
                    ans = true;
                    break 'outer;
                }
            }
        }

        for i in 5..N {
            for j in 5..N {
                if (0..6).filter(|x| S[i - x][j - x] == b'#').count() >= 4 {
                    ans = true;
                    break 'outer;
                }
            }
        }

        S = rotate(S);
    }
    println!("{}", if ans { "Yes" } else { "No" });
}

// ---------- begin rotate ----------
// 1 2
// 3 4
// =>
// 3 1
// 4 2

fn rotate<T>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if a.is_empty() {
        return vec![];
    }

    let h = a.len();
    let w = a[0].len();
    assert!(a.iter().all(|a| a.len() == w));

    let mut b = (0..w).map(|_| Vec::with_capacity(h)).collect::<Vec<_>>();
    for a in a.into_iter().rev() {
        for (b, a) in b.iter_mut().zip(a) {
            b.push(a)
        }
    }
    b
}
// ---------- end rotate ----------
