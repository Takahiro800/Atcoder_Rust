use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    };

    let a: Vec<char> = s.chars().collect();
    let b: Vec<char> = t.chars().collect();

    if !a.contains(&b[0]) {
        println!("{}", 2 * n);
        return;
    }

    let mut k: usize = n;
    loop {
        if k == 0 && a[n - 1] != b[0] {
            break;
        }

        if a.get(n - k..) == b.get(0..=k - 1) {
            println!("{}", 2 * n - k);
            return;
        }
        k -= 1;
    }
    println!("{}", 2 * n);
}
