#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
        mut R: [usize;N]
    };

    R.sort_unstable();
    let mut r = vec![R[0]];

    for i in 1..R.len() {
        if let Some(last) = r.last() {
            let num = last + R[i];
            r.push(num);
        }
    }

    for _ in 0..Q {
        input! {q: usize}
        let ans = binary_search(&r, q);
        println!("{}", ans + 1)
    }
}

fn binary_search(R: &Vec<usize>, n: usize) -> isize {
    if R[0] > n {
        return -1;
    }

    let mut l = 0;
    let mut r = R.len();
    let mut mid;

    while l + 1 < r {
        mid = (l + r) / 2;

        if R[mid] <= n {
            l = mid;
        } else {
            r = mid;
        }
    }

    l as isize
}
