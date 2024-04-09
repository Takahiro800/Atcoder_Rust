#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        mut A: [usize; N-1],
    };

    A.sort();

    let min = A[0];
    let max = A[N - 2];
    let current: usize = A.iter().sum::<usize>() - (min + max);

    if current > X {
        println!("0");
        return;
    }

    let diff = X - current;

    let ans;
    if diff <= min {
        ans = 0;
    } else if min < diff && diff <= max {
        ans = diff as isize;
    } else {
        ans = -1;
    }

    println!("{}", ans);
}
