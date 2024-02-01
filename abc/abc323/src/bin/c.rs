#![allow(non_snake_case)]
use std::isize;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize;M],
        S: [Chars;N]
    };

    let mut points: Vec<usize> = vec![];
    let mut problems: Vec<Vec<usize>> = vec![];

    for (i, s) in S.iter().enumerate() {
        let mut p = i + 1;
        let mut r = vec![];

        for (j, s) in s.iter().enumerate() {
            if s == &'o' {
                p += &A[j];
            } else {
                r.push(A[j])
            }
        }
        points.push(p);
        r.sort_unstable();
        r.reverse();
        problems.push(r.clone());
    }
    let m = points.iter().max().unwrap();
    for i in 0..N {
        if &points[i] == m {
            println!("{}", 0)
        } else {
            let mut diff = (m - points[i]) as isize;
            let mut ans = 0;
            let ps = &problems[i];
            while diff > 0 {
                diff -= ps[ans] as isize;
                ans += 1;
            }
            println!("{}", ans)
        }
    }
}
