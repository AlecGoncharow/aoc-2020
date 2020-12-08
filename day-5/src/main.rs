use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entries: Vec<&str> = contents.split("\n").collect();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut ids = [false; (127 * 8) + 1];

    for entry in entries {
        let mut rows = (0, (1 << 7) - 1);
        let mut cols = (0, (1 << 3) - 1);

        for ch in entry.chars() {
            println!("[{}] rows: {:?} | cols: {:?}", ch, rows, cols);
            match ch {
                'F' => {
                    let diff = rows.1 - rows.0;
                    if diff == 1 {
                        rows.1 = rows.0;
                    } else {
                        rows.1 = rows.1 - (diff / 2);
                        if diff % 2 == 1 {
                            rows.1 -= 1;
                        }
                    }
                }
                'B' => {
                    let diff = rows.1 - rows.0;
                    if diff == 1 {
                        rows.0 = rows.1;
                    } else {
                        rows.0 = rows.0 + (diff / 2);

                        if diff % 2 == 1 {
                            rows.0 += 1;
                        }
                    }
                }
                'L' => {
                    let diff = cols.1 - cols.0;
                    if diff == 1 {
                        cols.1 = cols.0;
                    } else {
                        cols.1 = cols.1 - (diff / 2);
                        if diff % 2 == 1 {
                            cols.1 -= 1;
                        }
                    }
                }
                'R' => {
                    let diff = cols.1 - cols.0;
                    if diff == 1 {
                        cols.0 = cols.1;
                    } else {
                        cols.0 = cols.0 + (diff / 2);
                        if diff % 2 == 1 {
                            cols.0 += 1;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        let id = (rows.0 * 8) + cols.0;
        println!("[{}] rows: {:?} | cols: {:?}", id, rows, cols);
        ids[id] = true;

        if id > p1 {
            p1 = id;
        }
    }

    println!("[part 1] id: {}", p1);

    let mut last = 0;
    for (i, id) in ids.iter().enumerate() {
        if !id {
            if i - 1 != last {
                p2 = i;
                break;
            }

            last = i;
        }
    }
    println!("[part 2] my seat: {}", p2);

    Ok(())
}
