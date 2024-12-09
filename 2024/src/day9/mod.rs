use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[allow(unused)]
pub fn run() {
    let mut reader = aoc::read_file("src/day9/input-test.txt");
    let diskmap = read_into_diskmap();
    println!("Diskmap: {:?}", diskmap);

    let _checksum = part1(diskmap);
}

fn part1(diskmap: Vec<i32>) -> i32 {
    let blocks = parse_blocks(diskmap);
    println!("blocks: {:?}", blocks);
    0
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
            blocks.push(Block::Empty(*val as usize));
        }

        is_file_block = !is_file_block
    }

    blocks
}

fn read_into_diskmap() -> Vec<i32> {
    let line = String::from("2333133121414131402");

    return line
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
}

//A block can either have a file or represent empty space
//Block::File(i32, usize) will contain the file ID, and its size
//Block::Empty(usize) will contain its size
#[derive(Debug)]
enum Block {
    File(i32, usize),
    Empty(usize),
}
