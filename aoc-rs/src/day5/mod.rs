use std::{
    collections::HashMap,
    fs::File,
    i32,
    io::{BufRead, BufReader},
};

// maybe lets first place all updates into a map
// update page is the key, value is the index
//
//then with each rule, we can lookup the values in the map
// if the left value or right value is not found, we continue?
// else, make sure the value returned from left value in map is greater then the map value of the
// right

pub fn run() {
    // first grab all the rules, seperated by |
    let reader = aoc_rs::read_file("src/day5/input.txt");

    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut invalid_updates: Vec<Vec<i32>> = Vec::new();

    let mut fixed_updates: Vec<Vec<i32>> = Vec::new();

    // side effects :/
    parse_input(reader, &mut rules, &mut updates);

    // first pass, get all failed
    for update in updates.iter() {
        let mut update_map: HashMap<i32, usize> = HashMap::new();
        // populate map
        for (idx, page) in update.iter().enumerate() {
            update_map.insert(*page, idx);
        }

        if let Some(_) = check_update(&rules, &update_map) {
            invalid_updates.push(update.clone());
        }
    }

    // second pass, fix all failed
    for update in invalid_updates.iter() {
        let mut update_map: HashMap<i32, usize> = HashMap::new();
        // populate map
        for (idx, page) in update.iter().enumerate() {
            update_map.insert(*page, idx);
        }

        // get the rule that failed, swap the incorrect values, try it again
        while let Some(rule) = check_update(&rules, &update_map) {
            let left = rule.left;
            let right = rule.right;

            let map_left = match update_map.get(&left) {
                Some(val) => *val,
                None => continue,
            };

            let map_right = match update_map.get(&right) {
                Some(val) => *val,
                None => continue,
            };

            // If left is after right, swap the indices to reconstruct
            update_map.insert(left, map_right);
            update_map.insert(right, map_left);
        }

        // update fixed, rebuild the vec from the map
        let max_index = update_map.values().copied().max().unwrap_or(0);
        let mut fixed = vec![0; max_index + 1];

        for (k, v) in update_map {
            fixed[v] = k;
        }

        fixed_updates.push(fixed);
    }

    let mut sum = 0;
    for update in fixed_updates.iter() {
        let mid = update.len() / 2;
        sum += update[mid];
    }

    println!("sum: {sum}");
}

// checks a given update against a given set of rules, returning the failed rule if it doesnt pass
fn check_update(rules: &Vec<Rule>, update_map: &HashMap<i32, usize>) -> Option<Rule> {
    for rule in rules.iter() {
        let rule = rule;
        let left = rule.left;
        let right = rule.right;

        let map_left = update_map.get(&left);
        let map_right = update_map.get(&right);

        let map_left = match map_left {
            Some(val) => *val,
            None => continue,
        };

        let map_right = match map_right {
            Some(val) => *val,
            None => continue,
        };

        // both values are valid, rule is valid

        // this means, the right rule value was found in the update, before the left, making
        // the rule failed
        if map_right < map_left {
            return Some(rule.clone());
        }
    }

    None
}

fn parse_input(reader: BufReader<File>, rule_buf: &mut Vec<Rule>, update_buf: &mut Vec<Vec<i32>>) {
    let mut parsing_rules = true;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let rule = parse_rule(line);
            rule_buf.push(rule);
        } else {
            let update = parse_update(line);
            update_buf.push(update);
        }
    }
}

fn parse_rule(line: String) -> Rule {
    let split: Vec<&str> = line.split("|").collect();
    Rule {
        left: split[0].parse().unwrap(),
        right: split[1].parse().unwrap(),
    }
}

fn parse_update(line: String) -> Vec<i32> {
    let mut update: Vec<i32> = Vec::new();
    let split: Vec<&str> = line.split(",").collect();

    for val in split {
        let val = val.parse().unwrap();
        update.push(val);
    }

    update
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    left: i32,
    right: i32,
}
