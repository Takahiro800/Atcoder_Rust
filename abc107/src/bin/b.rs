use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [String; h],
    }

    let rows: Vec<Vec<char>> = a.iter().map(|row| row.chars().collect()).collect();

    let mut new_x = vec![];

    for x in 0..w {
        let mut count = 0;
        for y in 0..h {
            if rows[y][x] == '.' {
                count += 1;
            }
        }
        if count == h {
            continue;
        }
        new_x.push(x);
    }

    let new_w = new_x.len();
    let mut new_y = vec![];

    for y in 0..h {
        let mut count = 0;
        for x in 0..new_w {
            if rows[y][new_x[x]] == '.' {
                count += 1;
            }
        }
        if count == new_w {
            continue;
        }
        new_y.push(y);
    }

    for y in new_y {
        println!("{}", new_x.iter().map(|&i| rows[y][i]).collect::<String>());
    }
}
