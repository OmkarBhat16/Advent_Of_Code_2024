use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input05.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    let mut hmap: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut tests: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 5 {
            let nums: Vec<i32> = line
                .split("|")
                .map(|f| f.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();
            if let Some(mut get_vec) = hmap.get_mut(&nums[0]) {
                get_vec.push(nums[1]);
            } else {
                hmap.insert(nums[0], vec![nums[1]]);
            }
        } else if line.len() > 5 {
            let nums: Vec<i32> = line
                .split(",")
                .map(|f| f.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();
            tests.push(nums);
        }
    }
    for key in hmap.keys() {
        println!("{key} : {:?}", hmap.get(&key));
    }
    let mut valids: Vec<Vec<i32>> = Vec::new();

    for test in tests {
        let mut flag = true;
        println!("{:?}", test);
        'outer: for i in (1..test.len()).rev() {
            let slice = &test[0..(i)];
            if let Some(nums) = hmap.get(&test[i]) {
                for num in nums {
                    if slice.contains(num) {
                        flag = false;
                        break 'outer;
                    }
                }
            }
        }
        if flag {
            valids.push(test);
        }
    }

    for valid in valids {
        if valid.len() > (valid.len() / 2) {
            result += valid[(valid.len() / 2)]
        }
    }
    println!("Final Result for day 05 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input05.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    let mut hmap: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut tests: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 5 {
            let nums: Vec<i32> = line
                .split("|")
                .map(|f| f.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();
            if let Some(mut get_vec) = hmap.get_mut(&nums[0]) {
                get_vec.push(nums[1]);
            } else {
                hmap.insert(nums[0], vec![nums[1]]);
            }
        } else if line.len() > 5 {
            let nums: Vec<i32> = line
                .split(",")
                .map(|f| f.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>();
            tests.push(nums);
        }
    }
    for key in hmap.keys() {
        println!("{key} : {:?}", hmap.get(&key));
    }
    let mut invalids: Vec<Vec<i32>> = Vec::new();

    for test in tests {
        let mut flag = true;
        println!("{:?}", test);
        'outer: for i in (1..test.len()).rev() {
            let slice = &test[0..(i)];
            if let Some(nums) = hmap.get(&test[i]) {
                for num in nums {
                    if slice.contains(num) {
                        flag = false;
                        break 'outer;
                    }
                }
            }
        }
        if !flag {
            invalids.push(test);
        }
    }

    for mut invalid in invalids.iter_mut() {
        // println!("{:?}", invalid);
        let len = invalid.len();
        let mut modified = false;
        for i in 1..len {
            if i >= invalid.len() {
                break;
            }
            let slice = &invalid[i..];
            // println!("{:?}", slice);
            for num in slice {
                if let Some(get_vec) = hmap.get(num) {
                    if get_vec.contains(&invalid[i - 1]) {
                        invalid.push(invalid[i - 1]);
                        invalid.remove(i - 1);
                        modified = true;
                        break;
                    }
                }
            }
            if modified {
                break;
            }
        }
    }

    for invalid in invalids {
        result += invalid[invalid.len() / 2];
    }
    println!("Final Result for day 05 part 2 : {}", result);
}
