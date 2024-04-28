#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(usize,usize);N]
    };

    let mut ans = 0;

    for c in (0..N).combinations(2) {
        let p = XY[c[0]];
        let q = XY[c[1]];

        let x_diff = p.0.abs_diff(q.0);
        let y_diff = p.1.abs_diff(q.1);

        if x_diff % 2 != y_diff % 2 {
            continue;
        }

        ans += x_diff.max(y_diff);
    }

    println!("{}", ans);
}
