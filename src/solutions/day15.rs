use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input15.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    println!("Final Result for day 15 part 1 : {}", result);
}
pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input15.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    println!("Final Result for day 15 part 2 : {}", result);
}