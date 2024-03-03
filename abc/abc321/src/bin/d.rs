#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        M: usize,
        P: usize,
        mut A: [usize;N],
        mut B: [usize;M]
    };

    A.sort();
    B.sort();

    let mut cumulative_sum = vec![0; M + 1];

    for (i, b) in B.iter().enumerate() {
        cumulative_sum[i + 1] = cumulative_sum[i] + b;
    }

    let mut ans = 0;

    for a in A.iter() {
        if a > &P {
            ans += P * M;
        } else {
            let diff = P - a;
            let cnt = B.lower_bound(&diff);

            ans += (cumulative_sum[cnt] + a * cnt) + P * (M - cnt);
        }
    }
    println!("{}", ans);
}
