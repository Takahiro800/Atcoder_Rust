#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        _N: usize,
        K: usize,
        mut A: [usize;K]
    };

    match K % 2 {
        0 => {
            let chunks: Vec<_> = A.chunks(2).collect();
            let ans: usize = chunks.iter().map(|chunk| chunk[1] - chunk[0]).sum();

            println!("{}", ans)
        }
        1 => {
            let mut l = vec![0];
            let mut r = vec![0];

            for chunk in A.chunks_exact(2) {
                l.push(chunk[0].abs_diff(chunk[1]) + l.last().cloned().unwrap());
            }

            for chunk in A.rchunks_exact(2) {
                r.push(chunk[0].abs_diff(chunk[1]) + r.last().cloned().unwrap())
            }
            r.reverse();
            let ans = l.iter().zip(&r).map(|(l, r)| l + r).min().unwrap();

            println!("{}", ans)
        }
        _ => unreachable!(),
    };
}
