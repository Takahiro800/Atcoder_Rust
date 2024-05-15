#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };

    let MOD = 100_000_000;
    A.sort();
    let mut count = 0;

    for (i, &a) in A.iter().enumerate() {
        let res = MOD - a;
        let t = A.lower_bound(&res);
        if t < N {
            if t <= i {
                count += N - i - 1;
            } else {
                count += N - t;
            }
        }
    }
    let sum: usize = A.iter().sum();
    let ans = (sum * (N - 1)) - (10_usize.pow(8) * count);

    println!("{}", ans);
}
