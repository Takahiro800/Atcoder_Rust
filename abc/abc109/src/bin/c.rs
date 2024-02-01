use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        mut a: [usize; N],
    };

    a.push(X);
    a.sort();

    let mut diff = vec![];
    for i in 0..N {
        diff.push(a[i + 1] - a[i])
    }
    diff.sort();
    println!("{}", gcd_array(&diff));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    } else {
        gcd(b, a % b)
    }
}

fn gcd_array(arr: &Vec<usize>) -> usize {
    arr.iter().fold(arr[0], |result, &n| gcd(result, n))
}
