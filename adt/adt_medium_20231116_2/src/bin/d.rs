#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize;K],
        XY: [(i64, i64); N],
    };

    let lights: Vec<_> = A.iter().map(|i| XY[i - 1]).collect();
    let not_lights: Vec<_> = XY.into_iter().filter(|p| !lights.contains(p)).collect();
    let mut current_max = 0;

    for p in not_lights {
        if let Some(dis) = calc_dis(&lights, p) {
            if dis > current_max {
                current_max = dis;
            }
        }
    }

    let ans = calc_square(current_max);
    println!("{}", ans);
}

fn calc_dis(XY: &[(i64, i64)], p: (i64, i64)) -> Option<i64> {
    XY.iter()
        .map(|&(x, y)| {
            let dx = p.0 - x;
            let dy = p.1 - y;
            dx * dx + dy * dy
        })
        .min()
}

fn calc_square(num: i64) -> f64 {
    (num as f64).sqrt()
}
