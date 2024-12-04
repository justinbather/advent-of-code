use std::{
    cmp::{max, min},
    io::{BufRead, BufReader, Read},
    isize, vec,
};

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
    fn new(relative_col: i32, relative_row: i32) -> Option<Self> {
        match (relative_col, relative_row) {
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

pub fn run() {
    let f = aoc_rs::read_file("src/day4/input-test.txt");

    // lets grab each line first
    // should have a Vec of lines
    let buffers = split_all(f);
    let mut m_neighbors = 0;

    for (row, buffer) in buffers.iter().enumerate() {
        for (col, char) in buffer.iter().enumerate() {
            if char == "X" {
                println!("Found X: line {}, col {}", row, col);
                let neighbours = search(&buffers, "M", row, col);
                match neighbours {
                    Some(vals) => {
                        for _position in vals.iter() {
                            m_neighbors += 1;
                        }
                    }
                    None => {}
                };
            }
        }
    }

    println!("Found {m_neighbors}m's next to an X")

    //
    // lets then just search through the lines and get the number of x's
    // Done
    //
    // then lets grab the number of x's with m's next to them
    // Done
    //
    // Now that we can reliably gte the Ms next to or across from an X, we just need to specify in
    // what direction its from
    //
    // Maybe the Search fn can return a struct that has col, row, and direction from ref?
    //  eg T (top), B(bottom), L(left), R(right), TR(top right), TL, BR(bottom right), BL
    //
    //  if we find an x, we can look for an m in any direction
    //
    //  if we get 1 m returned with position, and its direction = T, we can tell search to then
    //  look for an A with direction T and only return then
    //  alternatively, since we have a direction, we could just calculate? and if M was found with
    //  direction = Down at col 10, row 6, we then know A would need to be at col 10 row 7,
    //
    //  this would maybe make the code simpler? initial search for m returns all possibilities,
    //  which for each result, we call find for the next letter, calculated by the direction of M
    //  from X
    //
    //
}

fn rec_find(
    haystack: &Vec<Vec<String>>,
    targets: Vec<String>,
    direction: Direction,
    ref_col: usize,
    ref_row: usize,
) -> bool {
}

// left off building out this function
// based on a direction, and where the cursor is at a point in time, calculate where the next place
// to look
// The idea is that we find an x, then do a search for all surrounding ms
// for all the ms, we calculate the direction of the m from initial x, then start the find on the
// rest of the letters, calculating the next spot for the next letter
//
// alternative!
// we have x and find all the ms, based on the direciton of an m from an x, we calculate all the
// indexes right there and do a search. We first check the ending index is within bounds, if so,
// look for the first index calculated, if that has correct letter, continue on to the next, etc
// etc?
fn calc_abs_pos(
    direction: Direction,
    ref_row: usize,
    ref_col: usize,
    row_bounds: usize,
    col_bounds: usize,
    // add generic err type
) -> Result<(usize, usize)> {
    match direction {
        Direction::T => {
            if ref_row > 1 {
                Ok((ref_row - 1, 0))
            } else {
                Err(())
            }
        }
        Direction::B => {
            if ref_row < row_bounds {
                Ok((ref_row + 1, 0))
            } else {
                Err(())
            }
        }
        Direction::L => {
            if ref_col > 0 {
                Ok((0, ref_col - 1))
            } else {
                Err(())
            }
        }
        Direction::R => {
            if ref_col < col_bounds {
                Ok((0, ref_col + 1))
            } else {
                Err(())
            }
        }
        _ => {
            println!("Not supported");
            Err(())
        }
    }
}
fn search(
    haystack: &Vec<Vec<String>>,
    target: &str,
    ref_row: usize,
    ref_col: usize,
) -> Option<Vec<(usize, usize)>> {
    // for any given target, our valid search radius is ref_col +- 1, ref_row +- 1

    let mut positions: Vec<(usize, usize)> = Vec::new();
    for (row, buffer) in haystack.iter().enumerate() {
        for (col, char) in buffer.iter().enumerate() {
            // gone to far, get out
            if col > ref_col + 1 && row > ref_row + 1 {
                return None;
            }

            //if we are on our target and we are within valid range, add position to the vec

            // get the neighbor index or the edge (last row/col)
            let target_row_max: usize = min(ref_row + 1, haystack.len());
            let target_col_max: usize = min(ref_col + 1, buffer.len());

            // get the neighbour index or the first row (first row/col)
            let target_row_min: usize = max(ref_row as isize - 1, 0) as usize;
            let target_col_min: usize = max(ref_col as isize - 1, 0) as usize;

            if char == target
                && (row <= target_row_max && row >= target_row_min)
                && (col <= target_col_max && col >= target_col_min)
            {
                positions.push((row, col));
            }
        }
    }

    Some(positions)
}

fn split_all<T: std::io::Read>(f: BufReader<T>) -> Vec<Vec<String>> {
    let line_buffers: Vec<Vec<String>> = f
        .lines()
        .filter_map(|line| line.ok()) // Skip over any lines that fail to read
        .map(|line| {
            line.chars()
                .map(|c| c.to_string()) // Convert each char into a String
                .collect()
        })
        .collect();

    line_buffers
}
