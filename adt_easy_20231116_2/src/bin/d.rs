#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        A: [usize; k],
        XY: [(i64, i64); n]
    };

    let lights: Vec<_> = A.iter().map(|i| XY[i - 1]).collect();
    let mut current_max = 0;
    let not_A: Vec<_> = XY.into_iter().filter(|p| !lights.contains(p)).collect();

    for p in not_A {
        if let Some(dis) = calc_distance(&lights, p) {
            if dis > current_max {
                current_max = dis;
            }
        }
    }
    let ans = calc_square(current_max);
    println!("{}", ans)
}

fn calc_distance(XY: &[(i64, i64)], point: (i64, i64)) -> Option<i64> {
    XY.iter()
        .map(|&(x, y)| {
            let dx = point.0 - x;
            let dy = point.1 - y;
            dx * dx + dy * dy
        })
        .min()
}

fn calc_square(dis: i64) -> f64 {
    (dis as f64).sqrt()
}
