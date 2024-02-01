#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        H: usize,
        _W: usize,
        K: usize,
        mut S: [Bytes;H]
    };

    let mut ans = std::isize::MAX;
    for _ in 0..2 {
        for s in S.iter() {
            let mut a = vec![0; s.len() + 1];
            let mut b = vec![0; s.len() + 1];

            for (i, c) in s.iter().enumerate().rev() {
                a[i] = a[i + 1];
                b[i] = b[i + 1];

                if *c == b'o' {
                    a[i] += 1;
                } else if *c == b'x' {
                    b[i] += 1;
                }

                if i + K < a.len() && b[i] == b[i + K] {
                    ans = ans.min(K as isize - (a[i] - a[i + K]));
                }
            }
        }
        S = transpose(S);
    }
    if ans == std::isize::MAX {
        ans = -1;
    }
    println!("{}", ans);
}

fn transpose<T>(a: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if a.is_empty() {
        return a;
    }

    let h = a.len();
    let w = a[0].len();
    assert!(a.iter().all(|v| v.len() == w));
    let mut ta: Vec<_> = (0..w).map(|_| Vec::with_capacity(h)).collect();
    for a in a {
        for (ta, a) in ta.iter_mut().zip(a) {
            ta.push(a);
        }
    }
    ta
}
