use proconio::input;

fn main() {
    input! {
        x: i32
    };

    if x < 1200 {
        println!("{}", "ABC");
    } else {
        println!("{}", "ARC");
    }
}
