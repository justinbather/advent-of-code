use std::io::BufRead;

use aoc_rs::read_file;

pub fn run() {
    let f = read_file("src/day2/input-test.txt");

    let mut num_safe: i32 = 0;
    //let mut num_recalculated: i32 = 0;

    for line in f.lines() {
        let line = line.unwrap();

        let report = parse_line(line);
        let dir = parse_direction(&report);
        let dir = match dir {
            Some(d) => d,
            None => continue,
        };

        if parse_report2(&report, dir) {
            num_safe += 1;
        }
    }

    println!("{num_safe} reports are safe");
    //println!("{num_recalculated} reports were recalculated as safe");
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

fn parse_direction(report: &Vec<i32>) -> Option<Direction> {
    let mut num_asc: i32 = 0;
    let mut num_desc: i32 = 0;
    for (i, level) in report.iter().enumerate() {
        if report.len() >= i + 2 {
            let peek = &report[i + 1];
            if level > peek {
                num_desc += 1;
            }

            if level < peek {
                num_asc += 1;
            }
        }
    }

    if num_asc == 0 {
        return Some(Direction::DESC);
    }

    if num_desc == 0 {
        return Some(Direction::ASC);
    }

    if num_desc == 1 && num_asc > 1 {
        return Some(Direction::ASC);
    }

    if num_asc == 1 && num_desc > 1 {
        return Some(Direction::DESC);
    }

    None
}

fn should_skip(direction: Direction, curr: &i32, skip: &i32) -> bool {
    match direction {
        Direction::ASC => {
            if *skip >= *curr + 1 && *skip <= *curr + 3 {
                return true;
            }
        }
        Direction::DESC => {
            if *skip <= *curr - 1 && *skip >= *curr - 3 {
                return true;
            }
        }
    }

    false
}

fn parse_report2(report: &Vec<i32>, direction: Direction) -> bool {
    // no direction to be had, nothing to compare to, guessing its safe?
    if report.len() < 2 {
        return true;
    }
    let mut idx: usize = 0;
    let mut can_skip = true;
    while idx < report.len() - 2 {
        let curr = report[idx];
        let peek = report[idx + 1];

        match direction {
            Direction::ASC => {
                if can_skip {
                    if peek < curr {
                        if should_skip(direction, &curr, &report[idx + 2]) {
                            can_skip = false;
                            idx += 1;
                            println!("Skipping {curr}");
                            continue;
                        } else {
                            println!("Unsafe 1: {:#?}", report);
                            return false;
                        }
                    }

                    if !(peek >= curr + 1 && peek <= curr + 3) {
                        if should_skip(direction, &curr, &report[idx + 2]) {
                            can_skip = false;
                            idx += 1;
                            println!("Skipping {curr}");
                            continue;
                        } else {
                            println!("Unsafe 2: {:#?}", report);
                            return false;
                        }
                    }
                } else {
                    if peek < curr || !(peek >= curr + 1 && peek <= curr + 3) {
                        println!("Unsafe 3: {:#?}", report);
                        return false;
                    }
                }
            }

            Direction::DESC => {
                if can_skip {
                    if peek > curr {
                        if should_skip(direction, &curr, &report[idx + 2]) {
                            can_skip = false;
                            idx += 1;
                            println!("Skipping {curr}");
                            continue;
                        } else {
                            println!("Unsafe 4: {:#?}", report);
                            return false;
                        }
                    }

                    if !(peek <= curr - 1 && peek >= curr - 3) {
                        if should_skip(direction, &curr, &report[idx + 2]) {
                            can_skip = false;
                            idx += 1;
                            println!("Skipping {curr}");
                            continue;
                        } else {
                            println!("Unsafe: {:#?}", report);
                            return false;
                        }
                    }
                } else {
                    if peek > curr || !(peek <= curr - 1 && peek >= curr - 3) {
                        println!("Unsafe: {:#?}", report);
                        return false;
                    }
                }
            }
        }
        idx += 1;
    }

    println!("safe\n{:#?}\n", report);
    true
}

// report must be all ascending or all descending SAFE
// consecutive levels must differ by at least one but at most 3 to be SAFE
fn parse_report1(report: &Vec<i32>) -> bool {
    // no direction to be had, nothing to compare to, guessing its safe?
    if report.len() < 2 {
        return true;
    }

    // comparison
    let direction = set_direction(&report[0], &report[1]);
    let direction = match direction {
        Some(v) => v,
        // if we have a None, that means report.0 == report.1, meaning its unsafe
        None => {
            // try to get direction of next
            match set_direction(&report[0], &report[2]) {
                Some(v) => v,
                None => {
                    return false;
                }
            }
        }
    };

    // O(k) - linear
    //let mut unsafe_levels: i32 = 0;
    //let mut unsafe_idx: i64 = 0;

    parse_levels(&report, direction)
}

fn parse_levels(report: &Vec<i32>, direction: Direction) -> bool {
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
