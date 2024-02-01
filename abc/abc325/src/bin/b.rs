#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut WX: [(usize,usize);N]
    };

    let mut a = vec![0; 48];
    for (w, x) in WX {
        a[x] += w;
        a[x + 24] += w;
    }
    let ans = a.windows(9).map(|w| w.iter().sum::<usize>()).max().unwrap();
    println!("{}", ans);
}
