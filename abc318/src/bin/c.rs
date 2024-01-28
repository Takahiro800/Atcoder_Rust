#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        D: usize,
        P: usize,
        mut F: [usize;N]
    };

    F.sort_unstable();
    F.reverse();
    let mut count = 0;

    for chunk in F.chunks(D) {
        if chunk.iter().sum::<usize>() > P {
            count += 1;
        } else {
            break;
        }
    }

    let ans = if D * count < N {
        F[D * count..].iter().sum::<usize>() + count * P
    } else {
        count * P
    };
    println!("{}", ans);
}
