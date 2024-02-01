#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _N: usize,
        M: usize,
        S: String
    };

    let s: Vec<usize> = S
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut ans = 0;

    let mut n = 0;
    let mut l = 0;

    for num in s {
        match num {
            0 => {
                if n > M {
                    l += n - M;
                }
                if ans < l {
                    ans = l;
                }

                n = 0;
                l = 0;
            }
            1 => n += 1,
            2 => l += 1,
            _ => unreachable!(),
        }
    }
    if n > M {
        l += n - M;
    }
    if ans < l {
        ans = l;
    }

    println!("{}", ans)
}
