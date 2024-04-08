#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(isize, isize);N]
    };

    for i in 0..N {
        let j = (0..N)
            .max_by_key(|&j| {
                let a = XY[i];
                let b = XY[j];
                let dx = a.0 - b.0;
                let dy = a.1 - b.1;
                (dx * dx + dy * dy, !j)
            })
            .unwrap();
        println!("{}", j + 1);
    }
}
