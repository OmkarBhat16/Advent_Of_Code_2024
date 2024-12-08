#![allow(warnings)]
mod solutions;
use std::env;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args: Vec<String> = env::args().collect();
    let day = args.get(1).expect("enter the day");

    println!("day {} solution:", day);
    match day.as_str() {
        "1a" => solutions::day01::part1(),
        "1b" => solutions::day01::part2(),
        "2a" => solutions::day02::part1(),
        "2b" => solutions::day02::part2(),
        "3a" => solutions::day03::part1(),
        "3b" => solutions::day03::part2(),
        "4a" => solutions::day04::part1(),
        "4b" => solutions::day04::part2(),
        "5a" => solutions::day05::part1(),
        "5b" => solutions::day05::part2(),
        "6a" => solutions::day06::part1(),
        "6b" => solutions::day06::part2(),
        "7a" => solutions::day07::part1(),
        "7b" => solutions::day07::part2(),
        "8a" => solutions::day08::part1(),
        "8b" => solutions::day08::part2(),
        "9a" => solutions::day09::part1(),
        "9b" => solutions::day09::part2(),
        "10a" => solutions::day10::part1(),
        "10b" => solutions::day10::part2(),
        "11a" => solutions::day11::part1(),
        "11b" => solutions::day11::part2(),
        "12a" => solutions::day12::part1(),
        "12b" => solutions::day12::part2(),
        "13a" => solutions::day13::part1(),
        "13b" => solutions::day13::part2(),
        "14a" => solutions::day14::part1(),
        "14b" => solutions::day14::part2(),
        "15a" => solutions::day15::part1(),
        "15b" => solutions::day15::part2(),
        "16a" => solutions::day16::part1(),
        "16b" => solutions::day16::part2(),
        "17a" => solutions::day17::part1(),
        "17b" => solutions::day17::part2(),
        "18a" => solutions::day18::part1(),
        "18b" => solutions::day18::part2(),
        "19a" => solutions::day19::part1(),
        "19b" => solutions::day19::part2(),
        "20a" => solutions::day20::part1(),
        "20b" => solutions::day20::part2(),
        "21a" => solutions::day21::part1(),
        "21b" => solutions::day21::part2(),
        "22a" => solutions::day22::part1(),
        "22b" => solutions::day22::part2(),
        "23a" => solutions::day23::part1(),
        "23b" => solutions::day23::part2(),
        "24a" => solutions::day24::part1(),
        "24b" => solutions::day24::part2(),
        "25a" => solutions::day25::part1(),
        "25b" => solutions::day25::part2(),
        _ => println!("invalid day"),
    }
}
