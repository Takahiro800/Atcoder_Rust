#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        ST: [(usize, usize);N-1]
    };

    let mut curr = A[0];

    for i in 0..N {
        if i < N - 1 {
            let s = ST[i].0;
            let t = ST[i].1;
            let cnt = curr / s;
            curr = A[i + 1] + (cnt * t);
        }
    }

    println!("{}", curr);
}
