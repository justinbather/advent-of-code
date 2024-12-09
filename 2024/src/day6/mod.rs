#![allow(unused)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    isize, usize,
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

    fn unique_positions(&self) -> HashMap<(isize, isize), Direction> {
        self.stored_positions.clone()
    }
}

pub fn run() {
    let f = aoc::read_file("src/day6/input.txt");

    let grid = read_grid(f);
    let unique_positions = part1(&grid);
    println!("{unique_positions} unique positions");

    let num_obstacles = part2(&grid);
    println!("Can place {num_obstacles} unique obstacles to cause inifinite loops")
}

fn part2(grid: &Vec<Vec<String>>) -> i32 {
    // brute force it
    // run through part 1, getting all locations the guard visits
    // with the positions we can iterate over them
    // for each position, try adding an obstacle in that position and play it through
    // if it returns, we know it didnt work
    // if it gets stuck in a loop, it did. add one to the sum, and keep going

    //6,3

    let mut guard = init(&grid).expect("Couldnt find guard");

    let mut num_loops = 0;

    // play through to populate "walked path" map
    play(&mut guard, &grid, 100);

    let binding = guard.unique_positions();
    let positions: Vec<&(isize, isize)> = binding.keys().collect();

    // we only care about positions the guard can move, so iterate over them and try placing a
    // block in each place uniquely
    for (row, col) in positions.iter() {
        let mut new_grid = grid.clone();
        let mut new_guard = init(&new_grid).expect("Couldnt find guard");

        new_grid[*row as usize][*col as usize] = String::from("#");

        let is_loop = play(&mut new_guard, &new_grid, 10000000);
        if is_loop {
            println!("Loop caused by adding at row: {row} col: {col}");
            num_loops += 1;
        }
    }

    num_loops
}

fn part1(grid: &Vec<Vec<String>>) -> usize {
    let mut guard = init(&grid).expect("Couldnt find guard");

    play(&mut guard, grid, 10000001);

    guard.unique_positions().len()
}

fn play(guard: &mut Guard, grid: &Vec<Vec<String>>, max: i32) -> bool {
    // this counter is totally not great, just using it as a rough mean to exit with a value and
    // "prove" we in an infinite loop
    //
    // // would probably be better to just check if we turned at the same place in the past,
    // fugazie fugazie
    let mut counter = 0;
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
                //println!("Found the exit at row: {cur_row} col {cur_col}");
                return false;
            }

            let next_char = &grid[next_row as usize][next_col as usize];

            if next_char == "#" {
                guard.turn();
            } else {
                guard.walk();
            }
        } else {
            return false;
        }

        counter += 1;

        if counter == max {
            return true;
        }
    }
}

fn init(grid: &Vec<Vec<String>>) -> Option<Guard> {
    for (row_num, row) in grid.iter().enumerate() {
        for (col_num, char) in row.iter().enumerate() {
            if char == "^" {
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
