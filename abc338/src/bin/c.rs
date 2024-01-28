#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: [usize; N],
        A: [usize; N],
        B: [usize; N]
    }

    let mut ans = 0;
    for i in 0.. {
        if Q.iter().zip(A.iter()).any(|p| *p.0 < i * *p.1) {
            break;
        }

        let k = Q
            .iter()
            .zip(A.iter())
            .zip(B.iter())
            .filter(|p| p.1 > &0)
            .map(|((q, a), b)| (*q - i * *a) / *b)
            .min()
            .unwrap();
        ans = ans.max(i + k);
    }
    println!("{}", ans)
}
