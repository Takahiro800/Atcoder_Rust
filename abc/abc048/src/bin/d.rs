use proconio::input;

fn main() {
    input! {
        s: String
    };

    let s: Vec<char> = s.chars().collect();

    fn same(s: &Vec<char>) -> bool {
        s.first() == s.last()
    }

    fn even(s: &Vec<char>) -> bool {
        s.len() % 2 == 0
    }

    if same(&s) == even(&s) {
        println!("{}", "First");
    } else {
        println!("{}", "Second");
    }
}
