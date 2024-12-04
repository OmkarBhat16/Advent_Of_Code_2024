mod solutions;
use std::env;

fn main() {
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
        _ => println!("invalid day"),
    }
}
