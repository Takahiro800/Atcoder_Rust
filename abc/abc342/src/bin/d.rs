#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
    };
    let m = *A.iter().max().unwrap();
    A.retain(|&a| a > 0);

    let mut factor = (0..=m).collect::<Vec<_>>();

    for i in 2..=m {
        if factor[i] == i {
            for j in i..=(m / i) {
                factor[i * j] = i
            }
        }
    }

    let z = N - A.len();
    let mut ans = 0;

    if z > 0 {
        ans = z * (z - 1) / 2 + z * (N - z);
    }

    let mut cnt = vec![0; m + 1];

    for mut a in A {
        let mut k = 1;
        while a > 1 {
            // 素因数で割っていくので必ず1に辿り着く
            let p = factor[a];

            while a % (p * p) == 0 {
                a /= p * p;
            }

            if a % p == 0 {
                a /= p;
                k *= p;
            }
        }
        ans += cnt[k];
        cnt[k] += 1;
    }

    println!("{}", ans);
}
