// first iteration on part 2 completed in 375 microsecs, brute forces the dampener by trying all
// possiblities

use std::io::BufRead;

pub fn run() {
    let f = aoc::read_file("src/day2/input.txt");

    let mut num_safe: i32 = 0;

    for line in f.lines() {
        let line = line.unwrap();

        let report = parse_line(line);

        if parse_report(report) {
            num_safe += 1;
        }
    }

    println!("{num_safe} reports are safe");
}

fn parse_report(numbers: Vec<i32>) -> bool {
    if parse_levels(&numbers) {
        return true;
    }

    // retry k times
    for i in 0..numbers.len() {
        let mut temp = numbers.clone();
        temp.remove(i);
        if parse_levels(&temp) {
            return true;
        }
    }

    false
}

fn parse_levels(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }
    let mut increasing = true;
    let mut decreasing = true;

    for i in 1..numbers.len() {
        let gap = (numbers[i] - numbers[i - 1]).abs();
        if gap < 1 || gap > 3 {
            return false;
        }
        if numbers[i] > numbers[i - 1] {
            decreasing = false;
        }
        if numbers[i] < numbers[i - 1] {
            increasing = false;
        }
    }

    increasing || decreasing
}

fn parse_line(line: String) -> Vec<i32> {
    let mut report: Vec<i32> = Vec::new();

    for level in line.split_whitespace() {
        let level: i32 = level.parse().expect("To convert level to i32");
        report.push(level);
    }

    report
}
