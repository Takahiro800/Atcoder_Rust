use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [String; n],
    };

    for i in 1..n {
        let last_char = w[i - 1].chars().last().unwrap();
        let first_char = w[i].chars().next().unwrap();

        if last_char != first_char {
            println!("{}", "No");
            return;
        }

        if w.iter().filter(|&v| *v == w[i]).count() > 1 {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
