use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;
    let _re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for line in reader.lines() {
        // let chars: Vec<char> = line.unwrap().chars().collect();
        let line = line.unwrap();
        let mut itr = _re.captures_iter(&line);

        while let Some(caps) = itr.next() {
            // println!("{},{}", &caps[1], &caps[2]);
            result += &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
        }
    }
    println!("Final Result for day 03 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input03.txt").unwrap();
    let reader = BufReader::new(file);

    let mut result: i32 = 0;
    let _re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re1 = Regex::new(r"don't\(\)").unwrap();
    let re2 = Regex::new(r"do\(\)").unwrap();
    for line in reader.lines() {
        let mut line = line.unwrap();
        let dont_ranges: Vec<_> = re1.find_iter(&line).collect();
        let donts: Vec<i32> = dont_ranges.iter().map(|r| r.start() as i32).collect();
        let do_ranges: Vec<_> = re2.find_iter(&line).collect();
        let dos: Vec<i32> = do_ranges.iter().map(|r| r.start() as i32).collect();

        // println!("{:?}", donts);
        // println!("{:?}", dos);

        let mut pairs: Vec<(i32, i32)> = Vec::new();
        let mut dont_idx = 0;
        for i in 0..dos.len() {
            while dont_idx < donts.len() && donts[dont_idx] <= dos[i] {
                dont_idx += 1;
            }
            if dont_idx < donts.len() {
                pairs.push((dos[i], donts[dont_idx]));
            }
        }
        println!("old pairs : {:?}", pairs);

        loop {
            let mut flag = 0;
            for i in 1..pairs.len() {
                if i < pairs.len() && pairs[i - 1].1 > pairs[i].0 {
                    pairs.push((pairs[i - 1].0, cmp::max(pairs[i - 1].1, pairs[i].1)));
                    pairs.remove(i - 1);
                    pairs.remove(i - 1);
                    pairs.sort();
                    flag = 1;
                }
            }
            if flag == 0 {
                break;
            }
        }
        println!("new pairs : {:?}\n", pairs);

        let mut new_line = String::new();
        for pair in pairs {
            new_line.push_str(&line[pair.0 as usize..pair.1 as usize]);
        }

        let mut itr = _re.captures_iter(&new_line);

        while let Some(caps) = itr.next() {
            result += caps[1].parse::<i32>().unwrap() * caps[2].parse::<i32>().unwrap();
        }
    }

    println!("Final Result for day 03 part 2 : {}", result);
}
