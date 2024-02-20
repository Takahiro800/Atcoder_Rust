#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize;N],
        X: [usize;Q]
    };
    A.sort();
    let sum = A
        .iter()
        .scan(0, |sum, &a| {
            *sum += a;
            Some(*sum)
        })
        .collect::<Vec<usize>>();

    for x in X {
        let p = A.lower_bound(&x);
        let mut ans = 0;

        if p == 0 {
            ans += sum[N - 1] - x * N;
        } else {
            ans += x * p - sum[p - 1];
            ans += (sum[N - 1] - sum[p - 1]) - (x * (N - p));
        }
        println!("{}", ans);
    }
}
