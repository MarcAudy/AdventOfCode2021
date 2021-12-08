use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day8_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut result = 0;

    for l in buffered.lines()
    {
        let line = l?;
        let parts = line.split(" | ").collect::<Vec<&str>>();
        //let unique_digits = parts[0].split_whitespace().collect::<Vec<&str>>();
        let output_value = parts[1].split_whitespace().collect::<Vec<&str>>();
        println!("{:?}", output_value);
        println!("{:?}", output_value.iter().map(|x| x.len()).collect::<Vec<usize>>());

        result = result + output_value.iter().fold(0, |result, value| if value.len() == 2 || value.len() == 3 || value.len() == 4 || value.len() == 7 { result + 1 } else { result });
    }

    println!("{}", result);

    Ok(())
}