#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        H: [usize;N]
    };

    let h = H[0];

    let mut ans = -1;

    for i in 1..N {
        if H[i] > h {
            ans = i as isize + 1;
            break;
        }
    }
    println!("{}", ans);
}
