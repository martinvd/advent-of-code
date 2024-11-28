fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);

    println!("Part 1: {}", output);
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();

            (digits.first().unwrap().to_string() + &digits.last().unwrap().to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}
