use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    };

    let mut a: Vec<char> = s.chars().collect();
    let mut b: Vec<char> = t.chars().collect();
    a.sort();
    b.sort();
    b.reverse();

    let sorted_s: String = a.into_iter().collect();
    let sorted_t: String = b.into_iter().collect();

    if sorted_s < sorted_t {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
