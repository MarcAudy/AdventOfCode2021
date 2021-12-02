use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "d:\\AdventOfCode2021\\src\\day1_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    const WINDOWSIZE: usize = 3;

    let mut vals = Vec::new();
    let mut increases = 0;
    for l in buffered.lines() {
        let val = l?.parse::<i32>().unwrap();
        if vals.len() == WINDOWSIZE
        {
            let prev_sum: i32 = vals.iter().sum();
            vals.rotate_left(1);
            vals[WINDOWSIZE - 1] = val;
            let cur_sum: i32 = vals.iter().sum();
            if cur_sum > prev_sum
            {
                increases = increases + 1;
            }
        }
        else
        {
            vals.push(val);
        }
    }

    println!("{}", increases);

    Ok(())
}