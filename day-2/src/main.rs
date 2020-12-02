use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Policy {
    min: u8,
    max: u8,
    letter: char,
}

impl From<&str> for Policy {
    fn from(s: &str) -> Self {
        let mut split = s.split_whitespace();

        let mut num_split = split.next().unwrap().split('-');
        let letter = split.next().unwrap().parse::<char>().unwrap();

        let min = num_split.next().unwrap().parse::<u8>().unwrap();
        let max = num_split.next().unwrap().parse::<u8>().unwrap();

        Self { min, max, letter }
    }
}

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        let mut split = s.split(':');
        let policy = Policy::from(split.next().unwrap());
        let password = String::from(split.next().unwrap().trim());

        Self { policy, password }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let entries: Vec<Entry> = contents
        .split("\n")
        .filter(|s| s.len() > 1)
        .map(|s| Entry::from(s))
        .collect();

    let mut valid = 0;
    for entry in &entries {
        let mut count = 0;

        for ch in entry.password.chars() {
            if ch == entry.policy.letter {
                count += 1
            }
        }

        if entry.policy.min <= count && count <= entry.policy.max {
            valid += 1
        }
    }

    println!("[part 1] valid count: {}", valid);

    let mut valid = 0;
    for entry in entries {
        let mut count = 0;

        let chars = entry.password.as_bytes();

        let one = entry.policy.min as usize - 1;
        let two = entry.policy.max as usize - 1;

        if chars[one] as char == entry.policy.letter {
            count += 1;
        }

        if chars[two] as char == entry.policy.letter {
            count += 1;
        }

        if count == 1 {
            valid += 1
        }
    }
    println!("[part 2] valid count: {}", valid);

    Ok(())
}
