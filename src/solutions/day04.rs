use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part1() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input04.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;
    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    // Determine row_size first
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    // Horizontal
    for line in &lines {
        let mut count = 0;
        let mut itr = re1.captures_iter(line);
        while let Some(_) = itr.next() {
            count += 1;
        }
        itr = re2.captures_iter(line);
        while let Some(_) = itr.next() {
            count += 1;
        }
        result += count;
    }

    let mut vec_mat: Vec<Vec<char>> = Vec::new();
    for line in &lines {
        vec_mat.push(line.chars().collect());
    }
    let rows = vec_mat.len();
    let cols = vec_mat[0].len();

    //Diagonal L to R
    for i in 0..rows {
        for j in 0..cols {
            if i < rows - 3 && j < cols - 3 {
                let test: String = vec![
                    vec_mat[i][j],
                    vec_mat[i + 1][j + 1],
                    vec_mat[i + 2][j + 2],
                    vec_mat[i + 3][j + 3],
                ]
                .iter()
                .collect();
                // println!("{test}");
                if test == "XMAS" || test == "SAMX" {
                    result += 1;
                }
            }
        }
    }

    for i in 0..rows {
        for j in (0..cols).rev() {
            if i < rows - 3 && j >= 3 {
                let test: String = vec![
                    vec_mat[i][j],
                    vec_mat[i + 1][j - 1],
                    vec_mat[i + 2][j - 2],
                    vec_mat[i + 3][j - 3],
                ]
                .iter()
                .collect();
                // println!("{test}");
                if test == "XMAS" || test == "SAMX" {
                    result += 1;
                }
            }
        }
    }

    // Vertical
    let transposed: Vec<String> = (0..cols)
        .map(|col| (0..rows).map(|row| vec_mat[row][col]).collect())
        .collect();

    for col in &transposed {
        let mut count = 0;
        let mut itr = re1.captures_iter(col);
        while let Some(_) = itr.next() {
            count += 1;
        }
        itr = re2.captures_iter(col);
        while let Some(_) = itr.next() {
            count += 1;
        }
        result += count;
    }

    println!("Final Result for day 04 part 1 : {}", result);
}

pub fn part2() {
    let file = File::open("/mnt/D/Coding/Advent_Of_Code_2024/src/inputs/input04.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result: i32 = 0;

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }
    let mut vec_mat: Vec<Vec<char>> = Vec::new();
    for line in &lines {
        vec_mat.push(line.chars().collect());
    }
    let rows = vec_mat.len();
    let cols = vec_mat[0].len();

    let mut hset: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..rows {
        for j in 0..cols {
            if i < rows - 2 && j < cols - 2 {
                let test: String =
                    vec![vec_mat[i][j], vec_mat[i + 1][j + 1], vec_mat[i + 2][j + 2]]
                        .iter()
                        .collect();
                println!("{test}");
                if test == "MAS" || test == "SAM" {
                    hset.insert((i as i32 + 1, j as i32 + 1));
                }
            }
        }
    }

    for i in 0..rows {
        for j in (0..cols).rev() {
            if i < rows - 2 && j >= 2 {
                let test: String =
                    vec![vec_mat[i][j], vec_mat[i + 1][j - 1], vec_mat[i + 2][j - 2]]
                        .iter()
                        .collect();
                if test == "MAS" || test == "SAM" {
                    if hset.insert((i as i32 + 1, j as i32 - 1)) == false {
                        result += 1
                    }
                }
            }
        }
    }

    println!("Final Result for day 04 part 2 : {}", result);
}
