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

    while let (Some(p), Some(q)) = (P.get_mut(i), Q.get_mut(j)) {
        let len = p.1.min(q.1);

        if p.0 == q.0 {
            ans += len;
        }

        p.1 -= len;
        q.1 -= len;
        if p.1 == 0 {
            i += 1
        }
        if q.1 == 0 {
            j += 1
        }
    }

    println!("{}", ans);
}
