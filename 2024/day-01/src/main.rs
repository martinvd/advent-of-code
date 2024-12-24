use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut left_numbers: Vec<u32> = Vec::new();
    let mut right_numbers: Vec<u32> = Vec::new();

    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let left = parts[0].parse::<u32>().unwrap();
        let right = parts[1].parse::<u32>().unwrap();

        left_numbers.push(left);
        right_numbers.push(right);
    }

    left_numbers.sort();
    right_numbers.sort();
    let mut sum_of_diffs = 0;

    for i in 0..left_numbers.len() {
        let diff_abs = (left_numbers[i] as i32 - right_numbers[i] as i32).abs();
        sum_of_diffs += diff_abs;
    }

    println!("{}", sum_of_diffs);
}
