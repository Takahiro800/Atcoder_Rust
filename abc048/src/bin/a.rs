use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: String,
        B: String,
        C: String
    };

    let a: char = char0(A);
    let b: char = char0(B);
    let c: char = char0(C);

    let chars = vec![a, b, c];
    let abc: String = chars.into_iter().fold(String::new(), |mut str, c| {
        str.push(c);
        str
    });

    println!("{}", abc);
}

fn char0(x: String) -> char {
    return x.chars().nth(0).unwrap();
}
