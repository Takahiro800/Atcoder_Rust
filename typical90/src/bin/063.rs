#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        H: usize,
        W: usize,
        grid: [[usize; W]; H],
    };

    let mut ans = 1;

    for i in 1_usize..(1 << H) {
        let mut a = vec![];
        for j in 0..W {
            let mut b = vec![];
            for (k, p) in grid.iter().enumerate() {
                if i >> k & 1 == 1 {
                    b.push(p[j]);
                }
            }

            b.sort();
            let len = b.len();
            if b[0] == b[len - 1] {
                a.push(b[0]);
            }
        }
        a.sort();
        let h = i.count_ones() as usize;
        for (_, w) in run_length_encoding(a) {
            ans = ans.max(h * w);
        }
    }
    println!("{}", ans);
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
