#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize;M],
    };

    let mut cur = 0;

    for i in 0..N {
        let dif = A[cur] - (i + 1);
        println!("{}", dif);

        if dif == 0 {
            cur += 1;
        }
    }
}
