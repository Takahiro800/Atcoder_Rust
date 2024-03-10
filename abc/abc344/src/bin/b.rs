use std::str::FromStr;

fn main() {
    let mut v = vec![];
    let mut buffer = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut buffer) {
        if let Ok(n) = usize::from_str(buffer.trim()){
        v.push(n);
        buffer.clear();
        } else {
            break;
        }
    }

    v.reverse();
    for v in v.iter() {
        println!("{}", v);
    }
}
