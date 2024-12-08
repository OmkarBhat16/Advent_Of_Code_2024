use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input08.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;

    let mut hmap: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut hset: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in reader.lines().enumerate() {
        for (col, char) in line.unwrap().chars().enumerate() {
            if char != '.' {
                hmap.entry(char)
                    .or_insert_with(Vec::new)
                    .push((row as i32, col as i32));
            }
        }
    }

    for val in hmap {
        println!("{} : {:?}", val.0, val.1);
        for i in 0..val.1.len() {
            for j in i + 1..val.1.len() {
                let (r1, c1) = val.1[i];
                let (r2, c2) = val.1[j];
                let (r3, c3) = (r1 + (r1 - r2), c1 + (c1 - c2));
                let (r4, c4) = (r2 - (r1 - r2), c2 - (c1 - c2));

                while r3 >= 0 && r3 < 50 && c3 >= 0 && c3 < 50 {
                    hset.insert((r3, c3));
                    let (r3, c3) = (r3 + (r1 - r2), c3 + (c1 - c2));
                }
                while r4 >= 0 && r4 < 50 && c4 >= 0 && c4 < 50 {
                    hset.insert((r4, c4));
                    let (r4, c4) = (r4 - (r1 - r2), c4 - (c1 - c2));
                }
            }
        }
    }
    result = hset.len() as i32;
    println!("Final Result for day 08 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input08.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;

    let mut hmap: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut hset: HashSet<(i32, i32)> = HashSet::new();

    for (row, line) in reader.lines().enumerate() {
        for (col, char) in line.unwrap().chars().enumerate() {
            if char != '.' {
                hmap.entry(char)
                    .or_insert_with(Vec::new)
                    .push((row as i32, col as i32));
            }
        }
    }

    for val in hmap {
        println!("{} : {:?}", val.0, val.1);
        for i in 0..val.1.len() {
            for j in i + 1..val.1.len() {
                let (r1, c1) = val.1[i];
                let (r2, c2) = val.1[j];
                let (rdiff, cdiff) = (r1 - r2, c1 - c2);
                let (mut r3, mut c3) = (r1, c1);
                let (mut r4, mut c4) = (r2, c2);

                while r3 >= 0 && r3 < 50 && c3 >= 0 && c3 < 50 {
                    hset.insert((r3, c3));
                    (r3, c3) = (r3 + rdiff, c3 + cdiff);
                }
                while r4 >= 0 && r4 < 50 && c4 >= 0 && c4 < 50 {
                    hset.insert((r4, c4));
                    (r4, c4) = (r4 - rdiff, c4 - cdiff);
                }
            }
        }
    }
    result = hset.len() as i32;
    println!("Final Result for day 08 part 2 : {}", result);
}
