use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "d:\\AdventOfCode2021\\src\\day2_sample.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
  
    let mut part1_depth = 0;
    let mut part1_forward = 0;

    let mut part2_depth = 0;
    let mut part2_forward = 0;
    let mut part2_aim = 0;

    let r = Regex::new(r"(\w+) (\d+)").unwrap();

    for l in buffered.lines() {
        let line = l?;
        let caps = r.captures(&line).unwrap();
        let dir = caps[1].to_string();
        let val = caps[2].parse::<i32>().unwrap();

        if dir == "forward"
        {
            part1_forward = part1_forward + val;

            part2_forward = part2_forward + val;
            part2_depth = part2_depth + val * part2_aim;
        }
        else if dir == "down"
        {
            part1_depth = part1_depth + val;
            part2_aim = part2_aim + val;
        }
        else if dir == "up"
        {
            part1_depth = part1_depth - val;
            part2_aim = part2_aim - val;
        }
    }

    let part1_result = part1_depth * part1_forward;
    println!("Part1 - Depth: {} Forward: {} Result: {}", part1_depth, part1_forward, part1_result);

    let part2_result = part2_depth * part2_forward;
    println!("Part2 - Depth: {} Forward: {} Result: {}", part2_depth, part2_forward, part2_result);

    Ok(())
}