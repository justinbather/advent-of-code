#![allow(unused)]
use std::{
    fs::File,
    io::{BufReader, Read},
};

#[allow(unused)]
pub fn run() {
    let mut reader = aoc::read_file("src/day9/input-test.txt");
    let diskmap = read_into_diskmap(&mut reader);
    println!("Diskmap: {:?}", diskmap);

    let checksum = part1(diskmap);
    println!("Checksum: {checksum}")
}

fn part1(diskmap: Vec<i32>) -> u64 {
    let mut blocks = parse_blocks(diskmap);

    let compressed = compress(&mut blocks);
    println!("Compressed\n{compressed}\n");

    let checksum = get_checksum(compressed);

    checksum
}

fn get_checksum(input: String) -> u64 {
    let blocks: Vec<u64> = input
        .chars()
        .filter_map(|c| c.to_digit(10)) // Convert to digit if possible, could be a '.'
        .map(|d| d as u64)
        .collect();

    let mut checksum: u64 = 0;
    for (position, id) in blocks.iter().enumerate() {
        checksum += position as u64 * id;
    }

    checksum
}

// Sorts (compresses) a block vector, in place
fn compress(blocks: &Vec<Block>) -> String {
    let mut blocks: Vec<char> = blocks_to_string(blocks).chars().collect();
    let mut close: usize = 0;
    let mut far: usize = blocks.len() - 1;
    while close < far {
        match blocks[close] {
            // if its an empty spot, do nothing
            '.' => {}
            _ => {
                //close ++ and skip
                close += 1;
                continue;
            }
        }

        match blocks[far] {
            '.' => {
                far -= 1;
                continue;
            }
            _ => {}
        }

        //if we made it here, we have a empty spot in the close, and a file block in the far, meaning
        //we can safely swap and move the pointers in

        blocks.swap(close, far);
        close += 1;
        far -= 1;
    }

    return blocks.iter().collect();
}

fn parse_blocks(diskmap: Vec<i32>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    // flag to know what kind of block we are parsing
    let mut is_file_block = true;

    let mut curr_id = 0;
    for val in diskmap.iter() {
        if is_file_block {
            blocks.push(Block::File(curr_id, *val as usize));
            curr_id += 1;
        } else {
            if *val > 0 {
                blocks.push(Block::Empty(*val as usize));
            }
        }

        is_file_block = !is_file_block
    }

    blocks
}

fn read_into_diskmap(reader: &mut BufReader<File>) -> Vec<i32> {
    let mut buf = String::new();

    let _ = reader.read_to_string(&mut buf);
    let buf = buf.trim();

    return buf
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
}

fn blocks_to_string(blocks: &Vec<Block>) -> String {
    let mut s = String::new();
    for block in blocks.iter() {
        s += &block.to_string();
    }

    s
}

//A block can either have a file or represent empty space
//Block::File(i32, usize) will contain the file ID, and its size
//Block::Empty(usize) will contain its size
#[derive(Debug, Clone, Copy)]
enum Block {
    File(i32, usize),
    Empty(usize),
}

impl Block {
    fn to_string(&self) -> String {
        match self {
            Block::File(id, size) => {
                let mut s = String::new();
                for _ in 0..*size as i32 {
                    s += &id.to_string();
                }

                s
            }
            Block::Empty(size) => {
                let mut s = String::new();
                for _ in 0..*size as i32 {
                    s += ".";
                }

                s
            }
        }
    }
}
