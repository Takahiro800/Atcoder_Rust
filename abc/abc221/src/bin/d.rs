#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        AB: [(usize, usize);N]
    };

    let mut e = vec![];
    e.extend(AB.iter().map(|p| (p.0, 1)));
    e.extend(AB.iter().map(|p| (p.0 + p.1, -1)));
    e.sort();

    let mut ans = vec![0; N + 1];
    let mut count = 0;
    let mut pre_day = 0;

    for (curr_day, v) in e {
        ans[count as usize] += curr_day - pre_day;
        pre_day = curr_day;
        count += v;
    }

    println!("{}", ans[1..].iter().join(" "));
}
