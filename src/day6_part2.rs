use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const DAYS: u16 = 256;

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day6_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let initial_fish: Vec<usize> = buffered.lines().next().unwrap()?.split(',').map(|x| x.parse::<usize>().unwrap() ).collect(); 

    let mut fish_by_age: [u64; 9] = [0; 9];
    
    for fish_age in initial_fish
    {
        fish_by_age[fish_age] = fish_by_age[fish_age] + 1;
    }

    for _ in 0..DAYS
    {
        fish_by_age[7] = fish_by_age[7] + fish_by_age[0];
        fish_by_age.rotate_left(1);
    }

    println!("{}", fish_by_age.into_iter().sum::<u64>());

    Ok(())
}