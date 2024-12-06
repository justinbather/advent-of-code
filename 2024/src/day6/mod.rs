use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Guard {
    stored_positions: HashMap<(isize, isize), Direction>,
    position: (isize, isize),
    direction: Direction,
}

impl Guard {
    fn new(row: isize, col: isize) -> Self {
        let mut map: HashMap<(isize, isize), Direction> = HashMap::new();
        map.insert((row, col), Direction::Up);
        Guard {
            position: (row, col),
            stored_positions: map,
            direction: Direction::Up,
        }
    }

    // sets the guards position to the next spot infront of it, adds the position to the map and returns the new position
    fn walk(&mut self) -> (isize, isize) {
        let (new_row, new_col) = match self.direction {
            Direction::Up => {
                let (cur_row, cur_col) = self.position;
                self.position = (cur_row - 1, cur_col);
                self.position
            }
            Direction::Down => {
                let (cur_row, cur_col) = self.position;
                self.position = (cur_row + 1, cur_col);
                self.position
            }
            Direction::Left => {
                let (cur_row, cur_col) = self.position;
                self.position = (cur_row, cur_col - 1);
                self.position
            }
            Direction::Right => {
                let (cur_row, cur_col) = self.position;
                self.position = (cur_row, cur_col + 1);
                self.position
            }
        };
        self.stored_positions
            .insert((new_row, new_col), self.direction);

        self.position
    }

    // sets the direction of the guard returning the resulting direction
    fn turn(&mut self) {
        match self.direction {
            Direction::Up => {
                self.direction = Direction::Right;
            }
            Direction::Down => {
                self.direction = Direction::Left;
            }
            Direction::Left => {
                self.direction = Direction::Up;
            }
            Direction::Right => {
                self.direction = Direction::Down;
            }
        }
    }

    fn peek(&self) -> (isize, isize) {
        match self.direction {
            Direction::Up => {
                let (cur_col, cur_row) = self.position;
                (cur_col - 1, cur_row)
            }
            Direction::Down => {
                let (cur_col, cur_row) = self.position;
                (cur_col + 1, cur_row)
            }
            Direction::Left => {
                let (cur_col, cur_row) = self.position;
                (cur_col, cur_row - 1)
            }
            Direction::Right => {
                let (cur_col, cur_row) = self.position;
                (cur_col, cur_row + 1)
            }
        }
    }

    fn unique_positions(&self) -> usize {
        self.stored_positions.len()
    }
}

pub fn run() {
    let f = aoc::read_file("src/day6/input.txt");

    // read in
    // will have a 2d vec of strings
    let grid = read_grid(f);
    let unique_positions = part1(&grid);
    println!("{unique_positions} unique positions");
}

fn part2(grid: &Vec<Vec<String>>) -> i32 {
    // idk bro wtf
    //
    0
}

fn part1(grid: &Vec<Vec<String>>) -> usize {
    let mut guard = init(&grid).expect("Couldnt find guard");

    loop {
        let (next_row, next_col) = guard.peek();
        // check bounds
        if (next_row >= 0 && next_row <= grid.len() as isize)
            && (next_col >= 0 && next_col <= grid[0].len() as isize)
        {
            let (cur_row, cur_col) = guard.position;
            // at exit?
            if (cur_row == 0 || cur_row as usize == grid.len() - 1)
                || (cur_col == 0 || cur_col as usize == grid[0].len() - 1)
            {
                println!("Found the exit at row: {cur_row} col {cur_col}");
                break;
            }

            let next_char = &grid[next_row as usize][next_col as usize];

            if next_char == "#" {
                guard.turn();
            } else {
                guard.walk();
            }
        } else {
            break;
        }
    }

    guard.unique_positions()
}

fn init(grid: &Vec<Vec<String>>) -> Option<Guard> {
    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if char != "#" && char != "." {
                return Some(Guard::new(row_num as isize, col_num as isize));
            }
        }
    }

    None
}
fn read_grid(reader: BufReader<File>) -> Vec<Vec<String>> {
    let grid: Vec<Vec<String>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();

    grid
}
