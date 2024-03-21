#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };
    A.sort();
    A.dedup();

    let mut left = 0;
    let mut right = N + 1;

    while right > left + 1 {
        let mid = (left + right) / 2;
        let have = A.lower_bound(&(mid + 1));

        let count = N - have;
        let mut need = 0;

        if mid > have {
            need = (mid - have) * 2;
        };

        if count >= need {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
