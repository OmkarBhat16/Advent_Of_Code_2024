use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("./inputs/input01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut heap1: BinaryHeap<i32> = BinaryHeap::new();
    let mut heap2: BinaryHeap<i32> = BinaryHeap::new();

    for line in reader.lines() {
        let line_str = line.expect("invalid string");
        let nums: Vec<&str> = line_str.split_whitespace().collect();

        heap1.push(nums[0].parse().expect("invalid num"));
        heap2.push(nums[1].parse().expect("invalid num"));
    }

    let mut result: i32 = 0;
    while let (Some(a), Some(b)) = (heap1.pop(), heap2.pop()) {
        result += (a - b).abs();
    }

    println!("Final Result for day 01 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("./inputs/input01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut hmap: HashMap<i32, i32> = HashMap::new();
    let mut hset: HashSet<i32> = HashSet::new();

    for line in reader.lines() {
        let line_str = line.expect("invalid string");
        let nums: Vec<&str> = line_str.split_whitespace().collect();
        let num1: i32 = nums[0].parse().expect("invalid num");
        let num2: i32 = nums[1].parse().expect("invalid num");
        hset.insert(num1);

        hmap.entry(num2)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut result: i32 = 0;
    for val in &hset {
        let mut count = 0;
        if let Some(&existing_count) = hmap.get(&val) {
            count = existing_count;
        }
        result += val * count;
    }

    println!("Final Result for day 01 part 2 : {}", result);
}
