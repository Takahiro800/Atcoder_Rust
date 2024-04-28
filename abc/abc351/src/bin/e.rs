#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: isize,
        XY: [(isize,isize);N]
    };

    let mut evens = vec![];
    let mut odds = vec![];

    for &(x, y) in XY.iter() {
        let p = x + y;
        let q = x - y;

        if p % 2 == 0 {
            evens.push((p, q));
        } else {
            odds.push((p, q))
        }
    }

    let mut evens_x = evens.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    evens_x.sort();
    let mut evens_y = evens.iter().map(|&(_, y)| y).collect::<Vec<_>>();
    evens_y.sort();
    let mut odds_x = odds.iter().map(|&(x, _)| x).collect::<Vec<_>>();
    odds_x.sort();
    let mut odds_y = odds.iter().map(|&(_, y)| y).collect::<Vec<_>>();
    odds_y.sort();

    let p = vec![evens_x, evens_y, odds_x, odds_y];

    let mut ans = 0;

    for set in p.iter() {
        if set.len() > 1 {
            for (i, &v) in set.iter().enumerate() {
                ans += (2 * (i + 1) as isize - set.len() as isize - 1) * v;
            }
        }
    }

    ans /= 2;
    println!("{}", ans);
}
