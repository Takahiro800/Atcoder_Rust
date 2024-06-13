#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
    };

    let mut ans = 0;

    for i in 1..=N {
        let s = i.to_string();
        let mut a = vec![];

        for c in s.chars().rev() {
            a.push(c.to_digit(10).unwrap() as u64);
        }
        if a.contains(&7) {
            ans += 1;
            continue;
        }
        a = radix_conversion(&a, 10, 8);

        if a.contains(&7) {
            ans += 1;
            continue;
        }
    }
    ans = N - ans;
    println!("{}", ans);
}

// ---------- begin radix conversion(naive) ----------
fn radix_conversion(a: &[u64], p: u64, q: u64) -> Vec<u64> {
    assert!(p > 1 && q > 1);

    let mut b = vec![];
    for &a in a.iter().rev() {
        let mut carry = a;
        for b in b.iter_mut() {
            let v = *b * p + carry;
            *b = v % q;
            carry = v / q;
        }

        while carry > 0 {
            b.push(carry % q);
            carry /= q;
        }
    }

    if b.is_empty() {
        b.push(0);
    }

    b
}
// ----------- end radix conversion(naive) -----------
