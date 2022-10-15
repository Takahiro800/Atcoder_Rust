use proconio::input;

fn main() {
    input! {
        n: f64
    }

    let mut left = 0_f64;
    let mut right = 100_f64;

    for _ in 0..20 {
        let mid = (left + right) / 2.0;
        let v = mid.powf(3.0) + mid;

        if v < n {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("{}", left);
}
