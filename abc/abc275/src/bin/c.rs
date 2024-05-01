#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::{input, marker::Bytes};

fn main() {
    let N = 9;
    input! {
        S: [Bytes;N]
    };
    let mut ans = 0;

    for (dx, dy, x, y) in iproduct!(0..N, 1..N, 0..N, 0..N) {
        let mut dir = (dx, dy);
        let mut ok = true;
        let mut x = x;
        let mut y = y;
        for _ in 0..4 {
            ok &= x < N && y < N && S[x][y] == b'#';

            if !ok {
                break;
            }

            x += dir.0;
            y += dir.1;
            dir = (dir.1, !dir.0 + 1);
        }
        if ok {
            ans += 1;
        }
    }

    println!("{}", ans);
}
