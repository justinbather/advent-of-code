use core::panic;
use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let f = File::open("input.txt").expect("Shouldve opened input file");
    let f = BufReader::new(f);

    let (mut left, mut right) = parse(f);

    left.sort();
    right.sort();

    let total_dist = get_distance(&left, &right);
    let similarity = get_similarity(&left, &right);

    println!("The total distance is: {total_dist}");
    println!("The similarity is: {similarity}");
}

fn get_similarity(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    let mut similarity: i32 = 0;

    for left_val in left.iter() {
        match mp.get(left_val) {
            Some(v) => similarity += left_val * v,
            None => {
                let occurences = get_occurences(right, *left_val);
                mp.insert(*left_val, occurences);
                similarity += left_val * occurences;
            }
        }
    }

    similarity
}

// O(n)
fn get_occurences(list: &Vec<i32>, target: i32) -> i32 {
    let mut total: i32 = 0;

    for curr in list.iter() {
        if *curr == target {
            total += 1;
        }
    }

    total
}

// O(n)
fn get_distance(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    let mut total_dist: i32 = 0;

    for (idx, l) in left.iter().enumerate() {
        let min = min(l, &right[idx]);
        let max = max(l, &right[idx]);
        total_dist += max - min
    }

    total_dist
}

// Splits each line by whitespace, and adds the left and right values to respective vectors
// O(n)
fn parse(f: BufReader<File>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in f.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => panic!("Failed to read line, {e}"),
        };

        let seperated: Vec<&str> = line.split_whitespace().collect();

        let left_val: i32 = seperated[0].parse().unwrap();

        let right_val: i32 = seperated[1].parse().unwrap();

        left.push(left_val);
        right.push(right_val);
    }

    (left, right)
}
