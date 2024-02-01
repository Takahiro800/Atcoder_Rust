use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64
    };

    let inf = if a == 0 { -1 } else { (a - 1) / x };
    let sup: i64 = b / x;

    println!("{}", sup - inf);
}
