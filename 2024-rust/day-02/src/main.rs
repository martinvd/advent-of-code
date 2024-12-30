use std::fs;

fn is_safe_report(report: &str) -> bool {
    let levels = report.split_whitespace().collect::<Vec<&str>>();
    let mut levels_increasing = true;
    let mut levels_decreasing = true;
    let mut valid_distance = true;

    for window in levels.windows(3) {
        let a = window[0].parse::<i32>().unwrap();
        let b = window[1].parse::<i32>().unwrap();
        let c = window[2].parse::<i32>().unwrap();

        if b <= a || c <= b {
            levels_increasing = false;
        }

        if b >= a || c >= b {
            levels_decreasing = false;
        }

        let left_distance = (a - b).abs();
        let right_distance = (b - c).abs();
        let accepted_distance = 1..=3;

        if !accepted_distance.contains(&left_distance)
            || !accepted_distance.contains(&right_distance)
        {
            valid_distance = false;
        }
    }

    (levels_increasing || levels_decreasing) && valid_distance
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut safe_reports = 0;

    for report in contents.lines() {
        if is_safe_report(report) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {}", safe_reports);
}
