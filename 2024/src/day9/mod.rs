#![allow(unused)]
use std::{
    ffi::c_ushort,
    fs::File,
    io::{BufReader, Read},
    iter::StepBy,
};

#[allow(unused)]
pub fn run() {
    let mut reader = aoc::read_file("src/day9/input.txt");
    let diskmap = read_into_diskmap(&mut reader);

    let _p1_checksum = part1(&diskmap);
    // println!("Checksum: {p1_checksum}");

    //println!("Diskmap:\n{:?}", diskmap);
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

    compress_with_fragmentation(&mut blocks);

    get_checksum(blocks)
}

fn part2(diskmap: &Vec<i32>) -> u64 {
    // using the block structure so we know the contiguous size of a file or space,
    let mut blocks = diskmap_to_blocks(&diskmap);

    compress(&mut blocks);

    // convert back to optionals, so we have can have multi digit ids, only taking up one index
    let res = blocks_to_optional(blocks);

    get_checksum(res)
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

fn get_checksum_blocks(blocks: Vec<Block>) -> u64 {
    let mut checksum: u64 = 0;
    for (idx, block) in blocks.iter().enumerate() {
        match block {
            Block::File(id, _) => {
                checksum += *id as u64 * idx as u64;
            }
            Block::Empty(_) => {}
        }
    }

    checksum
}

fn blocks_to_optional(blocks: Vec<Block>) -> Vec<Option<i32>> {
    let mut result: Vec<Option<i32>> = Vec::new();
    for (idx, block) in blocks.iter().enumerate() {
        match block {
            Block::File(id, size) => {
                for _ in 0..*size {
                    result.push(Some(*id));
                }
            }
            Block::Empty(size) => {
                if *size == 0 {
                    continue;
                } else {
                    for _ in 0..*size {
                        result.push(None);
                    }
                }
            }
        }
    }

    result
}

// Sorts (compresses) a block vector, in place, keeping contiguous memory in tact
fn compress(blocks: &mut Vec<Block>) {
    // starting from the end, find the first block we need to move (farthest right)
    let mut far = blocks.len() - 1;
    while far > 0 {
        let mut file_size = 0;
        match blocks[far] {
            Block::File(_, size) => {
                file_size = size;
            }
            Block::Empty(_) => {
                far -= 1;
                continue;
            }
        }

        // we have the file that needs to be moved
        // loop through blocks to find the first spot it can be moved to

        // stay in bounds between start of blocks and the far pointer ie where the file is
        let mut cursor = 0;
        while cursor < far {
            match blocks[cursor] {
                Block::File(_, _) => {
                    cursor += 1;
                    continue;
                }
                Block::Empty(empty_chunk_size) => {
                    // cant fit here
                    if empty_chunk_size < file_size {
                        cursor += 1;
                        continue;
                    } else {
                        let remainder = empty_chunk_size - file_size;
                        if remainder == 0 {
                            blocks.swap(cursor, far);
                        } else {
                            // move replaced memory to the tail, add left over memory behind the
                            // file chunk
                            let remaining_chunk = Block::Empty(remainder);
                            let replaced_chunk = Block::Empty(empty_chunk_size - remainder);
                            // set current chunk to new size
                            blocks[cursor] = replaced_chunk;
                            // swap resized chunk with the file
                            blocks.swap(cursor, far);
                            // add the remaining empty chunk behind the file
                            blocks.insert(cursor + 1, remaining_chunk);

                            // normalize the far pointers position by making up for the increase in
                            // length, else we could skip a movable file
                            far += 1;
                        }
                        //trigger while loop to end
                        cursor = far;
                    }
                }
            }
        }
        far -= 1;
    }
}

// Sorts (compresses) a block vector, in place, with no concern about fragmenting (not keeping
// related data next to each other)
fn compress_with_fragmentation(blocks: &mut Vec<Option<i32>>) {
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
//Block::File(i32, i32) will contain the file ID, and its size
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
