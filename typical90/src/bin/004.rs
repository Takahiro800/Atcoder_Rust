#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
        mut A: [[usize;W];H]
    };

    let sum_H: Vec<usize> = A.iter().map(|a| a.iter().sum()).collect();
    let sum_W: Vec<usize> = (0..W).map(|j| A.iter().map(|row| row[j]).sum()).collect();

    let mut ans = vec![vec![0; W]; H];

    for (i, h) in sum_H.iter().enumerate() {
        for (j, w) in sum_W.iter().enumerate() {
            let sum = h + w - A[i][j];
            ans[i][j] = sum;
        }
    }

    for a in ans.iter() {
        println!("{}", a.iter().join(" "));
    }
}
