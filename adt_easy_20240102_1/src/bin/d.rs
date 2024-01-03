#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [Chars;H]
    };

    let mut ans = vec![0; W];

    for row in &grid {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '#' {
                ans[j] += 1;
            }
        }
    }

    for c in ans {
        println!("{}", c)
    }
}
