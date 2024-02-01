#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        mut A: [usize;N]
    };

    A.sort_unstable();
    let mut v = vec![];

    for a in &A {
        v.push(a - A[0])
    }

    let mut ans = 0;
    let mut r = 0;
    for i in 0..N {
        while r < N && v[r] - v[i] < M {
            r += 1;
        }

        ans = ans.max(r - i);
    }
    println!("{}", ans);
}
