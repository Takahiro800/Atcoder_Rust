#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        A: isize,
        B: isize,
        C: isize,
        D: isize,
    };

    let geta = 10_isize.pow(9);
    let (a, b, c, d) = (A + geta, B + geta, C + geta, D + geta);

    let calc = |x: isize, y: isize| -> isize {
        let mut ans = 0;
        for i in 0..x.min(4) {
            for j in 0..y.min(2) {
                let cnt = (1 + (x - i - 1) / 4) * (1 + (y - j - 1) / 2);
                ans += cnt;

                if i % 2 == j % 2 {
                    if i >= 2 {
                        ans -= cnt;
                    } else {
                        ans += cnt;
                    }
                }
            }
        }
        ans
    };

    let ans = calc(c, d) - calc(a, d) - calc(c, b) + calc(a, b);
    println!("{}", ans);
}
