use itertools::Itertools;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        K: usize,
        mut A: [usize; N]
    };

    if K >= N {
        println!("{}", vec![0; N].iter().join(" "));
        return;
    }

    for _ in 0..K {
        let _ = A.remove(0);
        A.push(0);
    }
    println!("{}", A.iter().join(" "));
}
