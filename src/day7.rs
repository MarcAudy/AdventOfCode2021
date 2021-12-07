use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PART1: bool = false;

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day7_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let initial_positions: Vec<i32> = buffered.lines().next().unwrap()?.split(',').map(|x| x.parse::<i32>().unwrap() ).collect(); 

    let low_pos = *initial_positions.iter().min().unwrap();
    let high_pos = *initial_positions.iter().max().unwrap();
   
    let mut costs: Vec<i32> = Vec::new();
    for pos in low_pos..high_pos+1
    {
        if PART1
        {
            costs.push(initial_positions.iter().fold(0, |cost, ip| cost + (ip-pos).abs()));
        }
        else
        {
            costs.push(initial_positions.iter().fold(0, |cost, ip| cost + (1..(ip-pos).abs()+1).sum::<i32>()));           
        }
    }

    println!("{}", costs.iter().min().unwrap());

    Ok(())
}