use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    };

    if a + 2 >= b {
        println!("{}", k + 1);
        return;
    }

    let exchange_num = (k - a + 1) / 2;
    let remark = (k - a + 1) % 2;

    let ans = (b - a) * exchange_num + a + remark;
    println!("{}", ans);
}
