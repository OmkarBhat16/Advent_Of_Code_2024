use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input02.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("invalid string");
        let nums: Vec<i32> = line
            .split(' ')
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();

        let mut flag: i32 = 0;
        for i in 0..nums.len() - 1 {
            let delta: i32 = (nums[i] - nums[i + 1]).abs();
            if delta > 3 || delta < 1 {
                break;
            }
            if nums[i] > nums[i + 1] {
                flag += 1;
            } else if nums[i] < nums[i + 1] {
                flag -= 1;
            }
        }
        if flag.abs() + 1 == nums.len() as i32 {
            result += 1;
        }
    }

    println!("Final Result for day 02 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input02.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("invalid string");
        let nums: Vec<i32> = line
            .split(' ')
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();
        if is_safe(&nums) == true {
            result += 1;
        }
    }

    println!("Final Result for day 02 part 2 : {}", result);
}

fn is_safe(arr: &Vec<i32>) -> bool {
    if is_valid(&arr) {
        return true;
    }

    for i in 0..arr.len() {
        let mut modified = arr.clone();
        modified.remove(i);
        if is_valid(&modified) {
            return true;
        }
    }

    false
}

fn is_valid(arr: &Vec<i32>) -> bool {
    let incr = arr.windows(2).all(|w| w[1] > w[0]);
    let decr = arr.windows(2).all(|w| w[1] < w[0]);
    let deltas = arr
        .windows(2)
        .all(|w| (w[1] - w[0]).abs() >= 1 && (w[1] - w[0]).abs() <= 3);
    return (incr || decr) && deltas;
}
