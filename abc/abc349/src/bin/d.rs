#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        L: usize,
        R: usize
    };

    let mut ans = vec![];

    let mut left = L;

    while left < R {
        let mut right;

        if left == 0 {
            right = R.next_power_of_two();
            if !R.is_power_of_two() {
                right /= 2;
            }
            ans.push((left, right));
            left = right;
            continue;
        } else {
            right = left.next_power_of_two();
            if left == right {
                right *= 2;
            }

            if R < right {
                if R % 2 == 0 {
                    ans.push((left, R));
                    left = R;
                } else {
                    right = R - 1;
                    ans.push((left, right));
                    left = right;
                }
                ans.push((left, left + 1));
                left += 1;
            } else {
                ans.push((left, right));
                left = right;
            }
        }
    }

    println!("{}", ans.len());
    for (l, r) in ans.iter() {
        println!("{} {}", l, r);
    }
}
