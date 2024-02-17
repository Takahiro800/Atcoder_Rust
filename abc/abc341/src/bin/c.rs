#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        H: usize,
        W: usize,
        _N: usize,
        mut T: Chars,
        S: [Chars;H]
    };

    let mut ans = 0;

    for i in 0..H {
        'outer: for j in 0..W {
            if S[i][j] == '#' {
                continue;
            }

            let mut k = i;
            let mut l = j;

            for c in T.iter() {
                (k, l) = match c {
                    'R' => (k, l + 1),
                    'L' => (k, l.wrapping_add(!0)),
                    'U' => (k.wrapping_add(!0), l),
                    'D' => (k + 1, l),
                    _ => unreachable!(),
                };

                if k >= H || k == 0 || l >= W || l == 0 {
                    continue 'outer;
                }
                if S[k][l] == '#' {
                    continue 'outer;
                }
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
