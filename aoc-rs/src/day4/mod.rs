use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    isize, usize,
};
pub fn run() {
    let f = aoc_rs::read_file("src/day4/input.txt");
    let buffers = split_all(f);

    let mut found = 0;
    let mut mp: HashMap<(isize, isize), i32> = HashMap::new();

    for (row, buffer) in buffers.iter().enumerate() {
        for (col, char) in buffer.iter().enumerate() {
            if char == "M" {
                //println!("Found X: line {}, col {}", row, col);
                let a_results = search_around(&buffers, "A", row, col);
                if !a_results.is_empty() {
                    for result in a_results.iter() {
                        let (_, a_dir, a_row, a_col) = result;

                        let s_results = search_around(
                            &buffers,
                            "S",
                            a_row.clone() as usize,
                            a_col.clone() as usize,
                        );
                        for s_result in s_results.iter() {
                            let (_, s_dir, _, _) = s_result;

                            if *s_dir == *a_dir {
                                println!("Found MAS, a at line:{a_row} col:{a_col}");
                                let exists = mp.get(&(*a_row, *a_col));

                                match exists {
                                    Some(v) => mp.insert((*a_row, *a_col), *v + 1),
                                    None => mp.insert((*a_row, *a_col), 1),
                                };
                            }
                        }
                    }
                }
            }
        }
    }
    for value in mp.into_values() {
        if value == 2 {
            found += 1;
        }
    }
    println!("Found: {found}");
}

fn split_all<T: std::io::Read>(f: BufReader<T>) -> Vec<Vec<String>> {
    let line_buffers: Vec<Vec<String>> = f
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    line_buffers
}

fn search_around(
    grid: &Vec<Vec<String>>,
    target: &str,
    row: usize,
    col: usize,
) -> Vec<(String, Direction, isize, isize)> {
    let directions = [
        //(-1, 0),  // Up
        //(1, 0),   // Down
        //(0, -1),  // Left
        //(0, 1),   // Right
        (-1, -1), // Top-left diagonal
        (-1, 1),  // Top-right diagonal
        (1, -1),  // Bottom-left diagonal
        (1, 1),   // Bottom-right diagonal
    ];

    let mut results = Vec::new();

    for (dx, dy) in directions.iter() {
        let new_row = row as isize + dx;
        let new_col = col as isize + dy;

        // Ensure new indices are within bounds
        if new_row >= 0
            && new_row < grid.len() as isize
            && new_col >= 0
            && new_col < grid[new_row as usize].len() as isize
        {
            let char_at_pos = grid[new_row as usize][new_col as usize].clone();
            if char_at_pos == target {
                //println!("Found {target} at line {new_row}, col {new_col}");

                let direction =
                    Direction::new(new_col - col as isize, new_row - row as isize).unwrap();
                //println!("Found m in {:#?} direction", direction);

                results.push((char_at_pos, direction, new_row, new_col));
            }
        }
    }
    results
}

#[derive(Debug)]
enum Direction {
    T,
    B,
    L,
    R,
    TR,
    TL,
    BR,
    BL,
}

impl Direction {
    fn new(relative_col: isize, relative_row: isize) -> Option<Self> {
        match (relative_row, relative_col) {
            (-1, 0) => Some(Direction::T),
            (1, 0) => Some(Direction::B),
            (0, -1) => Some(Direction::L),
            (0, 1) => Some(Direction::R),
            (-1, 1) => Some(Direction::TR),
            (-1, -1) => Some(Direction::TL),
            (1, 1) => Some(Direction::BR),
            (1, -1) => Some(Direction::BL),
            _ => None, // Handle invalid directions
        }
    }
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::T, Direction::T) => true,
            (Direction::B, Direction::B) => true,
            (Direction::L, Direction::L) => true,
            (Direction::R, Direction::R) => true,
            (Direction::TL, Direction::TL) => true,
            (Direction::TR, Direction::TR) => true,
            (Direction::BR, Direction::BR) => true,
            (Direction::BL, Direction::BL) => true,
            _ => false,
        }
    }
}
