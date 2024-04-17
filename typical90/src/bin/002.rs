#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize
    }

    if N % 2 == 1 {
        return;
    }

    'outer: for p in (0..N).combinations(N / 2) {
        let mut a = vec![')'; N];
        p.iter().for_each(|&p| a[p] = '(');

        let mut pos = 0;

        for &a in a.iter() {
            if a == '(' {
                pos += 1;
            } else {
                pos -= 1;
            }

            if pos < 0 {
                continue 'outer;
            }
        }
        let s = a.into_iter().collect::<String>();
        println!("{}", s);
    }
}
