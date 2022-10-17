use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        A: [usize; n]
    }

    let half_index: usize = n / 2;

    let B = &A[..half_index];
    let C = &A[half_index..];

    let P = bit_search(B);
    let Q = bit_search(C);

    for p in P {
        let r = k - p;

        if binary_search(&Q, r) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(non_snake_case)]
/// bit全探索
fn bit_search(A: &[usize]) -> Vec<usize> {
    let mut X: Vec<usize> = vec![0; 0];

    for bit in 0..(1 << A.len()) {
        let mut sum = 0;
        for i in 0..(A.len()) {
            if bit & (1 << i) > 0 {
                sum += A[i];
            }
        }
        X.push(sum);
    }
    X.sort();
    X.dedup();
    return X;
}

#[allow(non_snake_case)]
/// 二分探索
fn binary_search(A: &Vec<usize>, target: usize) -> bool {
    let mut left = 0;
    let mut right = A.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if target < A[mid] && mid == 0 {
            return false;
        }

        if A[mid] == target {
            return true;
        } else if target < A[mid] {
            right = mid - 1;
        } else if A[mid] < target {
            left = mid + 1;
        }
    }

    return false;
}
