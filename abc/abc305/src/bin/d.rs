#![allow(non_snake_case)]
use proconio::input;
use std::collections::VecDeque;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        Q: usize,
        LR: [(usize,usize);Q]
    };

    let sleep: Vec<_> = A
        .windows(2)
        .enumerate()
        .map(
            |(i, window)| {
                if i % 2 == 1 {
                    window[1] - window[0]
                } else {
                    0
                }
            },
        )
        .collect();
    let mut sleep_sum: VecDeque<_> = sleep
        .iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect();

    sleep_sum.push_front(0);

    for (l, r) in LR {
        let s = A.lower_bound(&l);
        let e = A.lower_bound(&r);

        let mut diff = 0;

        if s % 2 == 0 {
            diff += A[s] - l;
        }
        if e % 2 == 0 {
            diff += r - A[e - 1];
        }

        let ans = sleep_sum[e - 1] - sleep_sum[s] + diff;
        println!("{}", ans)
    }
}
