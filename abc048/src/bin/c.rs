use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };

    let mut b = Vec::with_capacity(n);

    for i in 0..n {
        b.push(std::cmp::min(a[i], x))
    }

    for i in 0..n - 1 {
        if b[i] + b[i + 1] > x {
            b[i + 1] = x - b[i];
        }
    }

    let sum_a: usize = a.iter().sum();
    let sum_b: usize = b.iter().sum();

    println!("{}", sum_a - sum_b);
}
