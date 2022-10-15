use proconio::input;

fn main() {
    input! {
      n: usize,
      k: usize,
      a: [usize; n],
    }

    let mut left = 0;
    let mut right = 1000000000;

    while left < right {
        let mid = (left + right) / 2;
        let ans: bool = check_number(&a, mid, k);

        if ans {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    println!("{}", left);
}

fn check_number(a: &Vec<usize>, time: usize, k: usize) -> bool {
    let mut sum: usize = 0;

    for elem in a {
        sum += time / elem;
    }

    return sum >= k;
}
