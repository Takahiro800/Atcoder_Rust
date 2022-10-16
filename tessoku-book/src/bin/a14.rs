use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        k: usize,
        A: [usize;n],
        B: [usize;n],
        C: [usize;n],
        D: [usize;n],
    }

    let AB = all_sum(A, B);
    let CD = all_sum(C, D);

    for x in &AB {
        let remaining: usize = k - x;

        if binary_search(&CD, remaining) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

#[allow(non_snake_case)]
fn all_sum(X: Vec<usize>, Y: Vec<usize>) -> Vec<usize> {
    let mut Z: Vec<usize> = vec![0; 0];
    for x in &X {
        for y in &Y {
            Z.push(x + y);
        }
    }
    Z.sort();
    Z.dedup();
    return Z;
}

#[allow(non_snake_case)]
fn binary_search(X: &Vec<usize>, target: usize) -> bool {
    let mut left = 0;
    let mut right = X.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if target < X[mid] && mid == 0 {
            return false;
        }

        if X[mid] == target {
            return true;
        } else if target < X[mid] {
            right = mid - 1;
        } else if X[mid] < target {
            left = mid + 1;
        }
    }

    return false;
}
