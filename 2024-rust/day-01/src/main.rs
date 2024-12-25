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
    let mut similarity_score: u32 = 0;

    for i in 0..left_numbers.len() {
        let left_num = left_numbers[i];
        let diff_abs = (left_num as i32 - right_numbers[i] as i32).abs();
        sum_of_diffs += diff_abs;

        let left_in_right_count =
            right_numbers.iter().filter(|&num| *num == left_num).count() as u32;
        similarity_score += left_num * left_in_right_count;
    }

    println!("Part 1: {}", sum_of_diffs);
    println!("Part 2: {}", similarity_score);
}
