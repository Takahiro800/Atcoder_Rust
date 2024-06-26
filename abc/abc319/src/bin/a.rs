#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let ans = match S.as_str() {
        "tourist" => 3858,
        "ksun48" => 3679,
        "Benq" => 3658,
        "Um_nik" => 3648,
        "apiad" => 3638,
        "Stonefeang" => 3630,
        "ecnerwala" => 3613,
        "mnbvmar" => 3555,
        "newbiedmy" => 3516,
        "semiexp" => 3481,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
