use std::io::BufRead;

use aoc_rs::read_file;

pub fn run() {
    let f = read_file("src/day2/input.txt");

    let mut num_safe: i32 = 0;

    for line in f.lines() {
        let line = line.unwrap();

        let report = parse_line(line);
        if report_is_safe(report) {
            num_safe += 1;
        }
    }

    println!("{num_safe} reports are safe");
}

fn parse_line(line: String) -> Vec<i32> {
    let mut report: Vec<i32> = Vec::new();

    for level in line.split_whitespace() {
        let level: i32 = level.parse().expect("To convert level to i32");
        report.push(level);
    }

    report
}

#[derive(Copy, Clone)]
enum Direction {
    ASC,
    DESC,
}

// report must be all ascending or all descending SAFE
// consecutive levels must differ by at least one but at most 3 to be SAFE
fn report_is_safe(report: Vec<i32>) -> bool {
    // no direction to be had, nothing to compare to, guessing its safe?
    if report.len() < 2 {
        return true;
    }

    // comparison
    let direction = set_direction(&report[0], &report[1]);
    let direction = match direction {
        Some(v) => v,
        // if we have a None, that means report.0 == report.1, meaning its unsafe
        None => return false,
    };

    // O(k) - linear
    for (idx, curr) in report.iter().enumerate() {
        if report.len() >= idx + 2 {
            let peek = report[idx + 1];

            if !is_safe(direction, &curr, &peek) {
                return false;
            }
        }
    }

    true
}

fn is_safe(direction: Direction, curr: &i32, peek: &i32) -> bool {
    match direction {
        Direction::ASC => return *peek >= *curr + 1 && *peek <= *curr + 3,
        Direction::DESC => return *peek <= *curr - 1 && *peek >= *curr - 3,
    }
}

fn set_direction(curr: &i32, next: &i32) -> Option<Direction> {
    // asc
    if curr < next {
        Some(Direction::ASC)
    // desc
    } else if curr > next {
        Some(Direction::DESC)
    } else {
        // curr == peek, report is unsafe
        None
    }
}
