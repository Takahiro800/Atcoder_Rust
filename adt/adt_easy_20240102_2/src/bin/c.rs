#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(i64, i64); N]
    };

    let mut max_dis = 0;

    for i in 0..N {
        for j in i + 1..N {
            let p = XY[i];
            let q = XY[j];

            let dis = calc_dis(p, q);

            if dis > max_dis {
                max_dis = dis
            }
        }
    }

    println!("{}", calc_squart(max_dis))
}

fn calc_dis(p: (i64, i64), q: (i64, i64)) -> i64 {
    let dx = p.0 - q.0;
    let dy = p.1 - q.1;

    dx * dx + dy * dy
}

fn calc_squart(n: i64) -> f64 {
    (n as f64).sqrt()
}
