use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "d:\\AdventOfCode2021\\src\\day3_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
  
    let mut count = 0;
    let mut bitcounts = Vec::new();

    for l in buffered.lines() {
        let line = l?;
        let mut i = 0;
        for c in line.chars()
        {
            if c == '1'
            {
                if count == 0
                {
                    bitcounts.push(1);
                }
                else
                {
                    bitcounts[i] = bitcounts[i] + 1;
                }
            }
            else if count == 0
            {
                bitcounts.push(0);
            }
            i = i + 1;
        }
        count = count + 1;
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..bitcounts.len()
    {
        let bc = bitcounts[i];
        if bc > count / 2
        {
            gamma = gamma + (1 << bitcounts.len()-i-1); 
        }
        else
        {
            epsilon = epsilon + (1 << bitcounts.len()-i-1); 
        }
    }

    println!("{}", gamma * epsilon);

    Ok(())
}