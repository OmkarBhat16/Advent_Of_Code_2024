use std::fs::File;
use std::io::{BufRead, BufReader};

// fn helper(target: i64, arr: &Vec<i32>, mut idx: usize, mut cur: i64, str: String) -> bool {
//     if cur == target {
//         println!("{target} : {str}");
//         return true;
//     } else if cur > target || idx >= arr.len() {
//         return false;
//     } else {
//         return helper(
//             target,
//             &arr,
//             idx + 1,
//             cur + arr[idx] as i64,
//             format!("{}+{}", str, arr[idx]),
//         ) || helper(
//             target,
//             &arr,
//             idx + 1,
//             cur * arr[idx] as i64,
//             format!("{}*{}", str, arr[idx]),
//         );
//     }
//     return false;
// }

// pub fn part1() {
//     let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input07.txt").unwrap();
//     let reader = BufReader::new(file);
//     let mut result: i64 = 0;
//     let mut vals: Vec<(i64, Vec<i32>)> = Vec::new();
//     for line in reader.lines() {
//         let line = line.unwrap();
//         let splits: Vec<&str> = line.split(":").collect();
//         // println!("{:?}", splits);
//         vals.push((
//             splits[0].parse::<i64>().unwrap(),
//             splits[1]
//                 .split_whitespace()
//                 .map(|m| m.parse::<i32>().unwrap())
//                 .collect::<Vec<i32>>(),
//         ));
//     }
//     let mut counter = 0;
//     // for val in vals {
//     //     // println!("{} , {:?}", val.0, val.1);
//     //     if helper(val.0, &val.1, 1, val.1[0] as i64, val.1[0].to_string()) {
//     //         result += val.0;

//     //         counter += 1;
//     //     }
//     // }
//     for val in vals {
//         let mut idx: usize = 1;
//         let mut list: Vec<i64> = Vec::new();
//         list.push(val.1[0] as i64);
//         while !list.is_empty() && idx < val.1.len() {
//             if list.len() < 2 {
//                 let a = list.pop().unwrap_or(0);
//                 list.push(a + val.1[idx] as i64);
//                 list.push(a * val.1[idx] as i64);
//             } else {
//                 let a = list.pop().unwrap_or(0);
//                 let b = list.pop().unwrap_or(0);
//                 list.push(a + val.1[idx] as i64);
//                 list.push(a * val.1[idx] as i64);
//                 list.push(b + val.1[idx] as i64);
//                 list.push(b * val.1[idx] as i64);
//             }
//             idx += 1;

//             if list.contains(&val.0) {
//                 result += val.0;
//                 counter += 1;
//             }
//         }
//     }

//     println!("Final Result for day 07 part 1 : {} , {}", result, counter);
// }
//

pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input07.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i64 = 0;

    let mut vals: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split(":").collect();
        let arr: Vec<i64> = nums[1]
            .split_whitespace()
            .map(|e| e.parse::<i64>().unwrap())
            .collect();
        vals.push((nums[0].parse::<i64>().unwrap(), arr));
    }

    for val in vals {
        // println!("{} : {:?}", val.0, val.1);
        let mut list: Vec<i64> = Vec::new();
        list.push(val.1[0]);
        let mut idx = 1;
        while idx < val.1.len() {
            let mut temp: Vec<i64> = Vec::new();
            while list.len() > 0 {
                let a: i64 = list.pop().unwrap();
                temp.push(a + val.1[idx]);
                temp.push(a * val.1[idx]);
            }
            list.append(&mut temp);
            idx += 1;
        }
        if list.contains(&val.0) {
            result += val.0;
        }
    }

    println!("Final Result for day 07 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input07.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i64 = 0;

    let mut vals: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<&str> = line.split(":").collect();
        let arr: Vec<i64> = nums[1]
            .split_whitespace()
            .map(|e| e.parse::<i64>().unwrap())
            .collect();
        vals.push((nums[0].parse::<i64>().unwrap(), arr));
    }

    for val in vals {
        // println!("{} : {:?}", val.0, val.1);
        let mut list: Vec<i64> = Vec::new();
        list.push(val.1[0]);
        let mut idx = 1;
        while idx < val.1.len() {
            let mut temp: Vec<i64> = Vec::new();
            while list.len() > 0 {
                let a: i64 = list.pop().unwrap();
                temp.push(a + val.1[idx]);
                temp.push(a * val.1[idx]);
                temp.push(
                    (a.to_string() + &val.1[idx].to_string())
                        .parse::<i64>()
                        .unwrap(),
                );
            }
            list.append(&mut temp);
            idx += 1;
        }
        if list.contains(&val.0) {
            result += val.0;
        }
    }
    println!("Final Result for day 07 part 2 : {}", result);
}
