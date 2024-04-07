#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(isize, isize);N]
    };

    let mut v = vec![(0, 0); N];

    for i in 0..N {
        let p = XY[i];
        for j in i + 1..N {
            let q = XY[j];
            let d = distance(p, q);

            if v[i].1 == d {
                v[i] = (v[i].0.min(j + 1), d);
            } else if v[i].1 < d {
                v[i] = (j + 1, d);
            }

            if v[j].1 == d {
                v[j] = (v[j].0.min(i + 1), d);
            } else if v[j].1 < d {
                v[j] = (i + 1, d);
            }
        }
    }

    for (i, _) in v.iter() {
        println!("{}", i);
    }
}

fn distance(p: (isize, isize), q: (isize, isize)) -> isize {
    let diff_a = if p.0 >= q.0 { p.0 - q.0 } else { q.0 - p.0 };
    let diff_b = if p.1 >= q.1 { p.1 - q.1 } else { q.1 - p.1 };

    diff_a.pow(2) + diff_b.pow(2)
}
