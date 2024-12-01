use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// 2970688
fn main() {
    let f = File::open("input.txt").expect("Shouldve opened input file");
    let f = BufReader::new(f);

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in f.lines() {
        let line = match line {
            Ok(l) => {
                println!("line: {l}");
                l
            }
            Err(e) => panic!("Failed to read line, {e}"),
        };

        let seperated: Vec<&str> = line.split_whitespace().collect();

        let left_val: i32 = seperated[0]
            .parse()
            .expect("Shouldve converted left val to i32");

        let right_val: i32 = seperated[1]
            .parse()
            .expect("Shouldve converted right val to i32");

        left.push(left_val);
        right.push(right_val);
    }

    left.sort();
    right.sort();

    let mut total_dist: i32 = 0;

    for (idx, l) in left.iter().enumerate() {
        if l > &right[idx] {
            // println!("Adding {l} - {} = {}", &right[idx], l - &right[idx]);
            total_dist += l - &right[idx]
        } else if l < &right[idx] {
            // println!("Adding {} - {l} = {}", &right[idx], &right[idx] - l);
            total_dist += &right[idx] - l
        } else {
            continue;
        }
    }

    println!("The total distance is: {total_dist}");
}
