use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn read_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut result = Vec::new();

    for line in reader.lines() {
        result.push(line.unwrap());
    }

    result
}
