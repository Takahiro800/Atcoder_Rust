#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut A: [usize; N],
    };
    let mut count = 0;

    for _ in 0..Q {
        input! {
            T: usize,
            mut  x: usize,
            mut y: usize,
        }

        match T {
            1 => {
                x -= 1;
                y -= 1;

                x = (x + N - count) % N;
                y = (y + N - count) % N;

                let temp = A[x];
                A[x] = A[y];
                A[y] = temp;
            }
            2 => {
                count += 1;
            }
            3 => {
                x -= 1;
                let i = (x + N - count) % N;
                println!("{}", A[i]);
            }
            _ => unreachable!(),
        }
    }
}
