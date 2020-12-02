use std::fs::File;
use std::io::prelude::*;

fn bin_search(numbers: &[u32], mut lo: usize, mut hi: usize, lhs: u32, target: u32) -> Option<u32> {
    while lo < hi {
        let mid = (hi + lo) / 2;
        let other = numbers[mid];
        let result = lhs + other;
        println!(
            "low mid hi num other i,  {} {} {} {} {}",
            lo, mid, hi, lhs, other
        );

        if result == target {
            return Some(other);
        }

        if result > target {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut numbers: Vec<u32> = contents
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    numbers.sort();

    let target: u32 = 2020;

    let mut answer: (u32, u32) = (0, 0);
    for (i, number) in numbers.iter().enumerate() {
        let lo = i + 1;
        let hi = numbers.len();

        if let Some(other) = bin_search(&numbers, lo, hi, *number, target) {
            answer = (*number, other);
            break;
        }
    }

    println!(
        "[Part 1] Got answer: {:?}, type: {}",
        answer,
        answer.0 * answer.1
    );

    let mut answer: (u32, u32, u32) = (0, 0, 0);
    for (i, number) in numbers.iter().enumerate() {
        for (j, number_two) in numbers[i + 1..].iter().enumerate() {
            if number + number_two > target {
                break;
            }

            let lo = j + 1;
            let hi = numbers.len();

            if let Some(other) = bin_search(&numbers, lo, hi, number + number_two, target) {
                answer = (*number, *number_two, other);
                break;
            }
        }

        if answer.0 != 0 {
            break;
        }
    }

    println!(
        "[Part 2] Got answer: {:?}, type: {}",
        answer,
        answer.0 * answer.1 * answer.2
    );

    Ok(())
}
