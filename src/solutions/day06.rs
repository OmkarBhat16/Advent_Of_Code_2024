use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input06.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut pos: (i32, i32) = (89, 82);
    for (row, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();
        grid.push(line);
    }

    let dirs: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    loop {
        visited.insert((pos.0, pos.1));
        println!("{:?}", pos);
        println!("{} , {}", dirs[dir].0, dirs[dir].1);
        if (pos.0 + dirs[dir].0) < 130
            && (pos.1 + dirs[dir].1) < 130
            && (pos.0 + dirs[dir].0) >= 0
            && (pos.1 + dirs[dir].1) >= 0
        {
            if grid[(pos.0 + dirs[dir].0) as usize][(pos.1 + dirs[dir].1) as usize] == '#' {
                dir = (dir + 1) % 4;
            } else {
                pos.0 += dirs[dir].0;
                pos.1 += dirs[dir].1;
            }
        } else {
            break;
        }
    }

    result = visited.len() as i32;
    println!("Final Result for day 06 part 1 : {}", result);
}
pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input06.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    println!("Final Result for day 06 part 2 : {}", result);
}
