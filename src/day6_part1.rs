use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const DAYS: u8 = 80;

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day6_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut fish: Vec<u8> = buffered.lines().next().unwrap()?.split(',').map(|x| x.parse::<u8>().unwrap() ).collect(); 

    for _ in 0..DAYS
    {
        let fish_count = fish.len();
        for i in 0..fish_count
        {
            if fish[i] == 0
            {
                fish[i] = 6;
                fish.push(8);
            }
            else
            {
                fish[i] = fish[i] - 1;
            }
        }
    }

    println!("{}", fish.len());

    Ok(())
}