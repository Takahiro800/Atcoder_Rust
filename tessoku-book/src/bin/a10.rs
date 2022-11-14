use proconio::input;
use std::cmp;

fn main() {
    input! {
      n: usize,
      a: [usize; n],
      d: usize,
      lr: [(usize, usize); d]
    }

    let mut p = vec![0; n + 1];
    p[1] = a[0];
    for i in 1..n {
        p[i + 1] = cmp::max(p[i], a[i])
    }

    let mut q = vec![0; n + 1];
    q[n] = a[n - 1];
    for i in (1..n).rev() {
        q[i] = cmp::max(q[i + 1], a[i - 1]);
    }

    for i in 0..d {
        let left_max = p[lr[i].0 - 1];
        let right_max = q[lr[i].1 + 1];

        println!("{}", cmp::max(left_max, right_max));
    }
    print!("{}", ans)
}
