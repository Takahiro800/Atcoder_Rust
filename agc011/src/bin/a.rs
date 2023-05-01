use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        c: usize,
        k: usize,
        mut T: [usize; n]
    };

    T.sort();

    let mut count_bus = 1;
    let mut p_num = 1;
    let mut first = T[0];

    for i in 1..n {
        if first + k >= T[i] {
            if p_num < c {
                p_num += 1;
            } else {
                count_bus += 1;
                first = T[i];
                p_num = 1;
            }
        } else {
            count_bus += 1;
            first = T[i];
            p_num = 1;
        }
    }

    println!("{}", count_bus);
}
