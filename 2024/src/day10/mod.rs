use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

pub fn run() {
    let reader = aoc::read_file("src/day10/input-test.txt");
    let grid = parse_grid(reader);

    let res = part1(&grid);
}

fn parse_grid(reader: BufReader<File>) -> Vec<Vec<i32>> {
    let mut grid = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let row = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        grid.push(row);
    }

    grid
}

const TRAIL_HEAD: i32 = 0;

fn part1(grid: &Vec<Vec<i32>>) -> i32 {
    let mut highlights: Vec<Position> = Vec::new();
    // find all trail heads
    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, curr) in row.iter().enumerate() {
            if *curr == TRAIL_HEAD {
                let is_trail = search(grid, row_num, col_num, 1, &mut highlights);
                // found trail head. begin searching
                // look around in all possible directions (up, down, left, right), and recursively
                // call that fn with the next target
            }
        }
    }

    print_grid(grid, &highlights);
    0
}

fn search(
    grid: &Vec<Vec<i32>>,
    curr_row: usize,
    curr_col: usize,
    target: i32,
    highlights: &mut Vec<Position>,
) -> bool {
    let directions = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for direction in directions.iter() {
        let new_row = curr_row as i32 + direction.0;
        let new_col = curr_col as i32 + direction.1;

        if in_bounds(grid, new_row, new_col) {
            if grid[new_row as usize][new_col as usize] == target {
                highlights.push(Position {
                    row: new_row as usize,
                    col: new_col as usize,
                });
                if target == 9 {
                    return true;
                } else {
                    return search(
                        grid,
                        new_row as usize,
                        new_col as usize,
                        target + 1,
                        highlights,
                    );
                }
            }
        }
    }

    false
}

fn in_bounds(grid: &Vec<Vec<i32>>, row: i32, col: i32) -> bool {
    let row_max = grid.len() - 1;
    let col_max = grid[0].len() - 1;
    if (row <= row_max as i32 && row >= 0) && (col <= col_max as i32 && col >= 0) {
        return true;
    }

    false
}

fn print_grid(grid: &Vec<Vec<i32>>, highlights: &Vec<Position>) {
    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, num) in row.iter().enumerate() {
            let mut highlighted = false;
            for h in highlights.iter() {
                if h.row == row_num && h.col == col_num {
                    print!("\x1b[32m");
                    print!("{num}");
                    print!("\x1b[0m");
                    highlighted = true;
                    break;
                }
            }
            if !highlighted {
                print!("{num}");
            }
        }
        print!("\n")
    }
}

struct Position {
    row: usize,
    col: usize,
}
