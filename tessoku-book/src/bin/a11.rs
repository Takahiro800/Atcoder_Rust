use proconio::input;

fn main() {
    input! {
      n: usize,
      x: usize,
      a: [usize; n]
    }

    println!("{}", binary_search(n, x, a) + 1);
}

fn binary_search(n: usize, target: usize, a: Vec<usize>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = n - 1;

    while left <= right {
        let mid: usize = (left + right) / 2;

        if target == a[mid] {
            return mid as i32;
        } else if target < a[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return -2;
}
