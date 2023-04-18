use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        h: usize,
        w: usize,
        S: [String; h],
    };

    for y in 0..h {
        // let mut row = String::new();

        for x in 0..w {
            if S[y].chars().nth(x).unwrap() == '.' {
                continue;
            }

            let mut bol = false;

            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx * dy != 0 || dx == 0 && dy == 0 {
                        continue;
                    }

                    let xx = x as i32 + dx;
                    let yy = y as i32 + dy;

                    if xx >= 0 && xx < w as i32 && yy >= 0 && yy < h as i32 {
                        if S[yy as usize].chars().nth(xx as usize).unwrap() == '#' {
                            bol = true;
                            continue;
                        }
                    }
                }
            }

            if bol == false {
                println!("{}", "No");
                return;
            }
        }
    }
    println!("{}", "Yes");
}
