#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize); M]
    }

    let mut grapth = vec![vec![]; N + 1];

    for (a, b) in AB {
        grapth[a].push(b);
        grapth[b].push(a);
    }

    let mut count = 0;
    let mut ans = 0;

    for (i, g) in grapth.iter().enumerate() {
        if g.len() > count {
            count = g.len();
            ans = i;
        }
    }

    println!("{}", ans)
}
