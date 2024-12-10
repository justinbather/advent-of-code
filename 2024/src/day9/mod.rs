#![allow(unused)]
use std::{
    fs::File,
    io::{BufReader, Read},
    iter::StepBy,
};

#[allow(unused)]
pub fn run() {
    let mut reader = aoc::read_file("src/day9/input-test.txt");
    let diskmap = read_into_diskmap(&mut reader);

    let _p1_checksum = part1(&diskmap);
    // println!("Checksum: {p1_checksum}");

    println!("Diskmap:\n{:?}", diskmap);
    let p2_checksum = part2(&diskmap);
    println!("Checksum p2: {p2_checksum}")
}

fn part1(diskmap: &Vec<i32>) -> u64 {
    // parse diskmap into a vec of optional i32's
    // for map 123 we get:
    // [Some(0), None, None, Some(1), Some(1), Some(1)]
    // The optional holds the id of the file block, and there are n file blocks of the same id &&
    // n == size of the file block
    let mut blocks = diskmap_to_optionals(&diskmap);

    compress1(&mut blocks);

    let checksum = get_checksum(blocks);
    checksum
}

fn part2(diskmap: &Vec<i32>) -> u64 {
    let mut blocks = diskmap_to_blocks(&diskmap);
    println!("Blocks: ");
    println!("{:?}", blocks);

    let blocks = compress2(&mut blocks);
    let mut s = String::new();
    for block in blocks.iter() {
        s += &block.to_string();
    }

    println!("Compresed Blocks: {s}");
    println!("{:?}", blocks);
    0
}

fn get_checksum(blocks: Vec<Option<i32>>) -> u64 {
    let mut checksum: u64 = 0;
    for (idx, block) in blocks.iter().enumerate() {
        match block {
            Some(val) => checksum += idx as u64 * *val as u64,
            None => {}
        }
    }

    checksum
}

// Sorts (compresses) a block vector, in place
fn compress2(blocks: &mut Vec<Block>) -> Vec<Block> {
    // start from end, then look for the first space itll fit

    let mut new_blocks: Vec<Block> = blocks.clone();

    //iterate backwards
    for (idx, block) in blocks.iter().rev().enumerate() {
        match block {
            Block::File(outer_id, outer_size) => {
                for (inner_idx, inner_block) in blocks.iter().enumerate() {
                    match inner_block {
                        Block::File(_, _) => continue,
                        Block::Empty(size) => {
                            if size >= outer_size {
                                //we can swap these two
                                // if they are equal we just swap
                                // if size is bigger than file, we need to get the difference and
                                // insert it after the file

                                if size == outer_size {
                                    //clean swap
                                    new_blocks.swap(idx, inner_idx);
                                } else {
                                    // space is bigger than file
                                    let remainder = size - outer_size;
                                    new_blocks[inner_idx] = *block;
                                    new_blocks.insert(inner_idx + 1, Block::Empty(remainder));
                                }
                            }
                        }
                    }
                }
            }
            Block::Empty(_) => continue,
        }
    }
    new_blocks
}

// Sorts (compresses) a block vector, in place
fn compress1(blocks: &mut Vec<Option<i32>>) {
    let mut close: usize = 0;
    let mut far: usize = blocks.len() - 1;

    while close <= far {
        // get an empty block in close pointer, a file block in far pointer
        match blocks[close] {
            Some(_) => {
                close += 1;
                continue;
            }
            None => {}
        }

        match blocks[far] {
            Some(_) => {}
            None => {
                far -= 1;
                continue;
            }
        }

        //if we made it here, we have a empty spot in the close, and a file block in the far, meaning
        //we can safely swap and move the pointers in

        blocks.swap(close, far);
        close += 1;
        far -= 1;
    }
}

fn diskmap_to_blocks(diskmap: &Vec<i32>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    // flag to know what kind of block we are parsing
    let mut is_file_block = true;

    let mut curr_id = 0;
    for val in diskmap.iter() {
        if is_file_block {
            blocks.push(Block::File(curr_id, *val));
            curr_id += 1;
        } else {
            if *val > 0 {
                blocks.push(Block::Empty(*val));
            }
        }

        is_file_block = !is_file_block
    }

    blocks
}

fn diskmap_to_optionals(diskmap: &Vec<i32>) -> Vec<Option<i32>> {
    let mut blocks: Vec<Option<i32>> = Vec::new();

    // flag to know what kind of block we are parsing
    let mut is_file_block = true;

    let mut curr_id = 0;
    for val in diskmap.iter() {
        if is_file_block {
            for _ in 0..*val {
                blocks.push(Some(curr_id));
            }
            curr_id += 1;
        } else {
            if *val > 0 {
                for _ in 0..*val {
                    blocks.push(None);
                }
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

//A block can either have a file or represent empty space
//Block::File(i32, usize) will contain the file ID, and its size
//Block::Empty(usize) will contain its size
#[derive(Debug, Clone, Copy)]
enum Block {
    // id, size
    File(i32, i32),
    // size
    Empty(i32),
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
