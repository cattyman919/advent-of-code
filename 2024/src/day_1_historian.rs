use std::{
    collections::{BinaryHeap, HashMap},
    path::Path,
};

use crate::utils;

pub fn distance(left: u32, right: u32) -> u32 {
    if right >= left {
        right - left
    } else {
        left - right
    }
}

pub fn parse_lines(input_lines: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let mut left_res = Vec::new();
    let mut right_res = Vec::new();

    for line in input_lines {
        let line: Vec<_> = line.trim().split("   ").collect();

        if line.is_empty() {
            continue;
        }

        left_res.push(line[0].parse().expect("Expect a number"));
        right_res.push(line[1].parse().expect("Expect a number"));
    }
    (left_res, right_res)
}

pub fn run_part1<P: AsRef<Path>>(path: P) -> u32 {
    let (left_vec, right_vec) = parse_lines(utils::read_file(path));

    let mut left_heap: BinaryHeap<i32> = BinaryHeap::from(
        left_vec
            .into_iter()
            .map(|x| -(x as i32))
            .collect::<Vec<_>>(),
    );
    let mut right_heap: BinaryHeap<i32> = BinaryHeap::from(
        right_vec
            .into_iter()
            .map(|x| -(x as i32))
            .collect::<Vec<_>>(),
    );

    let mut sum: u32 = 0;

    while let Some(left_val) = left_heap.pop() {
        let left_val = left_val.unsigned_abs();
        let right_val = right_heap.pop().unwrap().unsigned_abs();
        sum += distance(left_val, right_val);
    }
    sum
}

pub fn run_part2<P: AsRef<Path>>(path: P) -> u32 {
    let (left_vec, right_vec) = parse_lines(utils::read_file(path));
    let mut map = HashMap::new();

    for num in right_vec.into_iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    left_vec
        .iter()
        .map(|val| val * map.get(val).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_historian_input_test_part1() {
        let path = Path::new("input")
            .join("day_1_historian")
            .join("test-input-1.txt");

        assert_eq!(run_part1(path), 11);
    }

    #[test]
    fn test_historian_input_test_part2() {
        let path = Path::new("input")
            .join("day_1_historian")
            .join("test-input-2.txt");

        assert_eq!(run_part2(path), 31);
    }
}
