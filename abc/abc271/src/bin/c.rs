#![allow(non_snake_case)]
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        mut A: [usize;N]
    };
    A.push(0);
    A.sort();
    let mut ans = 0;
    let mut left = 0;
    let mut right = N;

    let mut count = 0;

    for i in A.iter().skip(1) {
        if ans + 1 == *i {
            ans += 1;
            let pre = left;
            left = A.lower_bound(&(ans + 1)) - 1;
            count += left - pre - 1;
        } else {
            break;
        }
    }
    // println!(
    //     "ans: {}, left: {}, right: {}, count: {}",
    //     ans, left, right, count
    // );

    while left < right || count > 1 {
        if left < N && ans + 1 == A[left + 1] {
            // println!("left- ans: {}, left: {}, right: {}", ans, left, right);
            ans += 1;
            let pre = left;
            left = A.lower_bound(&(ans + 1)) - 1;
            count += left - pre - 1;
        } else if right > 1 && right + count > left + 1 {
            let mut rem = 2;
            let t = count.min(2);
            // println!("right- ans: {}, left: {}, right: {}", ans, left, right);
            ans += 1;
            rem -= t;
            count -= t;
            right -= rem;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
