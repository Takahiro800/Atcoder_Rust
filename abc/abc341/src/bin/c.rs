#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        H: usize,
        W: usize,
        _N: usize,
        T: Bytes,
        S: [Bytes;H]
    };

    let check = |x: usize, y: usize| -> bool {
        if S[x][y] == b'#' {
            return false;
        }

        let dir = [(1, 0), (0, 1), (!0, 0), (0, !0)];
        let p = "DRUL";

        let mut pos = (x, y);
        for &t in T.iter() {
            let k = p.bytes().position(|c| c == t).unwrap();
            pos.0 = pos.0.wrapping_add(dir[k].0);
            pos.1 = pos.1.wrapping_add(dir[k].1);

            if S[pos.0][pos.1] == b'#' {
                return false;
            }
        }
        true
    };

    let mut ans = 0;

    for i in 0..H {
        for j in 0..W {
            if check(i, j) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
