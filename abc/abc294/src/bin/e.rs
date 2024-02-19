#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _L: usize,
        N: usize,
        M: usize,
        mut P: [(usize,usize);N],
        mut Q: [(usize, usize);M]
    };

    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;

    while i < N && j < M {
        let a = &mut P[i];
        let b = &mut Q[j];
        let len = a.1.min(b.1);

        if a.0 == b.0 {
            ans += len;
        }

        a.1 -= len;
        b.1 -= len;
        if a.1 == 0 {
            i += 1;
        }
        if b.1 == 0 {
            j += 1;
        }
    }

    println!("{}", ans);
}
