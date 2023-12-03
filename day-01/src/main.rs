fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);

    println!("Output: {}", output);
}

fn part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut first_num = String::new();

        for char in line.chars() {
            if char.is_numeric() {
                first_num = char.to_string();
                break;
            }
        }

        let mut last_num = String::new();

        for char in line.chars().rev() {
            if char.is_numeric() {
                last_num = char.to_string();
                break;
            }
        }

        let combined = first_num.to_owned() + &last_num.to_owned();
        total += combined.parse::<u32>().unwrap();
    }

    total
}
