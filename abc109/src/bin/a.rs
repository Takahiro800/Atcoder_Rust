use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    if (a * b % 2) == 0 {
        println!("{}", "No");
    } else {
        println!("{}", "Yes");
    }
}
