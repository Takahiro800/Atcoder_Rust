#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        ABCD: [(i32, i32);4]
    };

    let A: (i32, i32) = ABCD[0];
    let B: (i32, i32) = ABCD[1];
    let C: (i32, i32) = ABCD[2];
    let D: (i32, i32) = ABCD[3];

    let AB = get_vec(A, B);
    let AD = get_vec(A, D);
    if calc_inner_product(AB, AD) <= 0 {
        println!("No");
        return;
    }

    let BA = get_vec(B, A);
    let BC = get_vec(B, C);
    if calc_inner_product(BC, BA) <= 0 {
        println!("No");
        return;
    }

    let CB = get_vec(C, B);
    let CD = get_vec(C, D);
    if calc_inner_product(CD, CB) <= 0 {
        println!("No");
        return;
    }

    let DA = get_vec(D, A);
    let DC = get_vec(D, C);
    if calc_inner_product(DA, DC) <= 0 {
        println!("No");
        return;
    }

    println!("Yes")
}

fn get_vec(s: (i32, i32), e: (i32, i32)) -> (i32, i32) {
    (e.0 - s.0, e.1 - s.1)
}

fn calc_inner_product(a: (i32, i32), b: (i32, i32)) -> i32 {
    a.0 * b.1 - a.1 * b.0
}
