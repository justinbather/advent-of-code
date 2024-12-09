#![allow(unused)]
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let reader = aoc::read_file("src/day7/input.txt");

    let calibrations = parse_calibrations(reader);

    let part1_results = part1(&calibrations);
    println!("Total part1 calibration result is: {part1_results}");

    let part2_results = part2(&calibrations);
    println!("Total part2 calibration result is: {part2_results}");
}

fn part2(calibrations: &Vec<Calibration>) -> i64 {
    let mut test_results = 0;
    for c in calibrations.iter() {
        let results = compute_combinations_with_pipe(&c.numbers);
        if results.contains(&c.test_val) {
            test_results += c.test_val;
        }
    }

    test_results
}

fn part1(calibrations: &Vec<Calibration>) -> i64 {
    let mut test_results = 0;
    for c in calibrations.iter() {
        let results = compute_combinations(&c.numbers);
        if results.contains(&c.test_val) {
            test_results += c.test_val;
        }
    }

    test_results
}

fn compute_combinations_with_pipe(numbers: &Vec<i64>) -> HashSet<i64> {
    fn recurse(index: usize, current_value: i64, numbers: &Vec<i64>, results: &mut HashSet<i64>) {
        if index == numbers.len() {
            results.insert(current_value);
            return;
        }

        recurse(index + 1, current_value + numbers[index], numbers, results);

        recurse(index + 1, current_value * numbers[index], numbers, results);

        recurse(
            index + 1,
            pipe(current_value, numbers[index]),
            numbers,
            results,
        );
    }

    let mut results = HashSet::new();
    if !numbers.is_empty() {
        recurse(1, numbers[0], numbers, &mut results);
    }

    results
}

fn compute_combinations(numbers: &Vec<i64>) -> HashSet<i64> {
    fn recurse(index: usize, current_value: i64, numbers: &Vec<i64>, results: &mut HashSet<i64>) {
        if index == numbers.len() {
            results.insert(current_value);
            return;
        }

        recurse(index + 1, current_value + numbers[index], numbers, results);

        recurse(index + 1, current_value * numbers[index], numbers, results);
    }

    let mut results = HashSet::new();
    if !numbers.is_empty() {
        recurse(1, numbers[0], numbers, &mut results);
    }

    results
}

fn pipe(left: i64, right: i64) -> i64 {
    let left = left.to_string();
    let right = right.to_string();

    let new = String::from(left + &right);

    new.parse().unwrap()
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
