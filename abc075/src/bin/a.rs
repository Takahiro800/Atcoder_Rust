use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: i32,
        B: i32,
        C: i32,
    };

    if A == B {
        println!("{}", C);
    } else if B == C {
        println!("{}", A);
    } else {
        println!("{}", B);
    }
}
