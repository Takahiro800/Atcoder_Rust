use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    println!("{}", if a <= b { a } else { a - 1 });
}
