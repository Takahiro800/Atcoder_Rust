use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        X: f32
    };

    println!("{}", X.round());
}
