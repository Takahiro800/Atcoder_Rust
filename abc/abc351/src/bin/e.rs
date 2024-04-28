#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: isize,
        XY: [(isize,isize);N]
    };

    let (even, odd): (Vec<_>, Vec<_>) = XY.iter().copied().partition(|&(x, y)| (x + y) % 2 == 0);
    let mut ans = 0_isize;

    for points in [even, odd] {
        let mut points = points
            .iter()
            .map(|&(x, y)| (x + y, x - y))
            .collect::<Vec<_>>();
        let n = points.len();

        points.sort_unstable();
        for (i, &(x, _)) in points.iter().enumerate() {
            let coeff = 2 * i as isize + 1 - (n as isize);
            ans += x * coeff;
        }

        points.sort_unstable_by_key(|&(_, y)| y);
        for (i, &(_, y)) in points.iter().enumerate() {
            let coeff = 2 * i as isize + 1 - (n as isize);
            ans += y * coeff;
        }
    }

    ans /= 2;
    println!("{}", ans);
}
