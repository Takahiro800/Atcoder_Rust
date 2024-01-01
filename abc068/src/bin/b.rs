use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    if n == 1 {
        println!("{}", 1);
        return;
    }

    let bin = format!("{:b}", n);
    let exp = bin.len() - 1;
    let ans = 1 << exp;
    println!("{}", ans);
}
