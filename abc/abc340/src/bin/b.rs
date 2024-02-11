#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        Q: usize,
    };

    let mut v = vec![];

    for _ in 0..Q {
        input! {
            q: usize,
            x: usize
        }

        match q {
            1 => v.push(x),
            2 => {
                let l = v.len();
                println!("{}", v[l - x]);
            }
            _ => unreachable!(),
        }
    }
}
