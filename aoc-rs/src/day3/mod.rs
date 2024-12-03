use std::io::Read;

use regex::Regex;

pub fn run() {
    let mut f = aoc_rs::read_file("src/day3/input.txt");

    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let results: Vec<(i32, i32)> = re
        .captures_iter(buf.as_str())
        .filter_map(|caps| {
            let first = caps.get(1)?.as_str().parse::<i32>().ok()?;
            let second = caps.get(2)?.as_str().parse::<i32>().ok()?;
            Some((first, second))
        })
        .collect();

    let mut sum = 0;
    for result in results {
        let (i, j) = result;
        println!("{i} x {j}");
        sum += i * j;
    }

    println!("{sum}");
}
