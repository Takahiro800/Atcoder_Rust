use proconio::input;

fn main() {
    input! {
      n: usize,
      mut a :  [usize; n],
      q: usize,
      x: [usize; q]
    }

    a.sort();

    for target in x {
        println!("{}", count_smaller(n, target, &a));
    }
}

fn count_smaller(n: usize, target: usize, a: &Vec<usize>) -> usize {
    let mut left: usize = 0;
    let mut right: usize = n - 1;

    while left <= right {
        let mid: usize = (left + right) / 2;

        if mid == 0 && target < a[mid] {
            return 0;
        }

        if target == a[mid] {
            return mid;
        } else if target < a[mid] {
            right = mid - 1;
        } else if a[mid] < target {
            left = mid + 1;
        }
    }

    return (left + right) / 2 + 1;
}
