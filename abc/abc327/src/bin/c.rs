#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: [[usize;9];9]
    }

    let mut c: Vec<Vec<_>> = vec![];
    for i in 0..9 {
        c.push((0..9).map(|j| A[i][j]).collect());
        c.push((0..9).map(|j| A[j][i]).collect());
        c.push(
            (0..9)
                .map(|j| A[i / 3 * 3 + j / 3][i % 3 * 3 + j % 3])
                .collect(),
        );
    }

    let ans = c.into_iter().all(|mut c| {
        c.sort_unstable();
        c.dedup();
        c.len() == 9
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
