#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        B: [[usize;M]; N]
    };

    if B.iter().all(|line| check_line(line)) && check_column(&B) {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn check_line(B: &Vec<usize>) -> bool {
    if B.first().unwrap() / 7 != B.last().unwrap() / 7 {
        return false;
    }

    B.windows(2).all(|w| w[1] - w[0] == 1)
}

fn check_column(B: &Vec<Vec<usize>>) -> bool {
    let len = B[0].len();

    for i in 0..B.len() - 1 {
        for j in 0..len {
            if !(B[i + 1][j] - B[i][j] == 7 && B[i + 1][j] % 7 == B[i][j] % 7) {
                return false;
            }
        }
    }

    true
}
