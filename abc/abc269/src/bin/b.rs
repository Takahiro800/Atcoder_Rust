use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
      s: [Bytes; 10],
    };
    let a = s.iter().position(|s| s.iter().any(|s| *s == b'#')).unwrap();
    let b = s
        .iter()
        .rposition(|s| s.iter().any(|s| *s == b'#'))
        .unwrap();
    let c = s[a].iter().position(|c| *c == b'#').unwrap();
    let d = s[a].iter().rposition(|c| *c == b'#').unwrap();
    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
