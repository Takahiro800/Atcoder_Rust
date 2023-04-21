use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };

    if n <= 2 {
        println!("{}", 1);
        return;
    }

    let mut ans = 1;
    let mut i = 1;

    loop {
        if i >= n - 1 {
            break;
        }

        if a[i] == a[i + 1] {
            a[i] = a[i - 1];
        }
        if (a[i] < a[i + 1] && a[i - 1] > a[i]) || (a[i] > a[i + 1] && a[i - 1] < a[i]) {
            ans += 1;
            i += 1;
        }
        i += 1;
    }

    println!("{}", ans)
}
