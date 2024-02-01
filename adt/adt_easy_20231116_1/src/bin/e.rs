#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut W: usize,
        mut ab: [(usize, usize); N]
    };

    // qを先に出すことで降順にsortしている
    ab.sort_by(|p, q| q.0.cmp(&p.0));

    let mut weight = 0;
    let mut value = 0;

    for cheeze in ab {
        let (v, w) = cheeze;

        let add_w = std::cmp::min(W - weight, w);
        weight += add_w;
        value += add_w * v;
    }

    println!("{}", value);
}
