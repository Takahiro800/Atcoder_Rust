#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: usize,
        Q: usize,
        mut A: [usize; N],
    };
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                for l in k + 1..N {
                    for m in l + 1..N {
                        if A[i] * A[j] % P * A[k] % P * A[l] % P * A[m] % P == Q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
