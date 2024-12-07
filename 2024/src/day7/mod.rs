use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let reader = aoc::read_file("src/day7/input.txt");

    let calibrations = parse_calibrations(reader);

    let calibration_results = part1(calibrations);
    println!("Total calibration result is: {calibration_results}")
}
fn part1(calibrations: Vec<Calibration>) -> i64 {
    let mut num_valid = 0;
    for c in calibrations.iter() {
        let results = compute_combinations(&c.numbers);
        if results.contains(&c.test_val) {
            num_valid += c.test_val;
        }
    }

    num_valid
}

fn compute_combinations(numbers: &Vec<i64>) -> HashSet<i64> {
    fn helper(index: usize, current_value: i64, numbers: &Vec<i64>, results: &mut HashSet<i64>) {
        // If we've processed all numbers, store the current result
        if index == numbers.len() {
            results.insert(current_value);
            return;
        }

        helper(index + 1, current_value + numbers[index], numbers, results);

        helper(index + 1, current_value * numbers[index], numbers, results);
    }

    let mut results = HashSet::new();
    if !numbers.is_empty() {
        helper(1, numbers[0], numbers, &mut results); // Start recursion from the first number
    }

    results
}

fn parse_calibrations(reader: BufReader<File>) -> Vec<Calibration> {
    let mut calibrations: Vec<Calibration> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let split: Vec<&str> = line.split(":").collect();
        let test_val: i64 = split[0].to_string().parse().unwrap();
        let numbers: Vec<i64> = split[1]
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        calibrations.push(Calibration { test_val, numbers });
    }

    calibrations
}

#[derive(Debug)]
struct Calibration {
    test_val: i64,
    numbers: Vec<i64>,
}
