use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        b: [String; h],
    };

    let mut new_b = vec![];

    for y in 0..h {
        let mut row = String::new();

        for x in 0..w {
            if b[y].chars().nth(x).unwrap() == '.' {
                let mut count = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let new_x = x as i32 + dx;
                        let new_y = y as i32 + dy;
                        if new_x >= 0 && new_x < w as i32 && new_y >= 0 && new_y < h as i32 {
                            if b[new_y as usize].chars().nth(new_x as usize).unwrap() == '#' {
                                count += 1;
                            }
                        }
                    }
                }
                row.push((count as u8 + b'0') as char);
            } else {
                row.push('#')
            }
        }
        new_b.push(row)
    }

    for row in new_b {
        println!("{}", row);
    }
}
