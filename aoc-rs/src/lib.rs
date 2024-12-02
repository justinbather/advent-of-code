use std::{fs::File, io::BufReader};

pub fn read_file(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
}
