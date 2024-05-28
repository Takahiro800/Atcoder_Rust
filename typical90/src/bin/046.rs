#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N],
        mut B: [usize; N],
        mut C: [usize; N],
    };
    let cons = 46;
    let mut ans = 0;

    let A = process_vector(&mut A, cons);
    let B = process_vector(&mut B, cons);
    let C = process_vector(&mut C, cons);

    for (a, count_a) in A.iter() {
        for (b, count_b) in B.iter() {
            for (c, count_c) in C.iter() {
                if (a + b + c) % cons == 0 {
                    ans += count_a * count_b * count_c;
                }
            }
        }
    }

    println!("{}", ans);
}

fn process_vector(vec: &mut Vec<usize>, cons: usize) -> Vec<(usize, usize)> {
    vec.iter_mut().for_each(|x| *x %= cons);
    vec.sort();
    run_length_encoding(vec.to_vec())
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
