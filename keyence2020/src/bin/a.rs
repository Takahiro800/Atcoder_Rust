use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };

    let m = std::cmp::max(h, w);
    println!("{}", (n + m - 1) / m);
}
