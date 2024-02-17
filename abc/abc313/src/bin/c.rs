#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };

    let ave = A.iter().sum::<usize>() / N;
    let surplus = A.iter().sum::<usize>() % N;
    A.sort();
    A.reverse();

    let mut cnt = 0;

    for (i, a) in A.iter().enumerate() {
        if i < surplus {
            cnt += a.abs_diff(ave + 1);
        } else {
            cnt += a.abs_diff(ave);
        }
    }
    let ans = cnt / 2;
    println!("{}", ans);
}
