use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        mut c: usize,
        k: usize,
    };

    for _i in 0..k {
        match [a, b, c].iter().max().unwrap() {
            v if v == &a => a *= 2,
            v if v == &b => b *= 2,
            v if v == &c => c *= 2,
            _ => (),
        }
    }

    println!("{}", a + b + c);
}
