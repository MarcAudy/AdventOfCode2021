use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn filter_entries(mut entries: Vec<Vec<u8>>, filter_bit_index: usize, target_bit: u8) -> i64
{
    let mut result: i64 = 0;

    let set_bit_count = entries.iter().filter(|&entry| entry[filter_bit_index]== target_bit).count();
    let bit = if set_bit_count > entries.len() - set_bit_count { 1 } 
              else if set_bit_count == entries.len() - set_bit_count { target_bit } 
              else { 0 };

    entries.retain(|entry| entry[filter_bit_index] == bit);

    if entries.len() == 1
    {
        for i in 0..entries[0].len()
        {           
            result = result + ((entries[0][i] as i64) << entries[0].len()-i-1); 
        }
    }
    else
    {
        result = filter_entries(entries, filter_bit_index + 1, target_bit);
    }

    return result;
}

fn main() -> Result<(), Error> {
    let path = "d:\\AdventOfCode2021\\src\\day3_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
  
    let mut entries = Vec::new();

    for l in buffered.lines() 
    {
        let line = l?;
        entries.push(Vec::new());

        for c in line.chars()
        {
            if c == '1'
            {
                entries.last_mut().unwrap().push(1);
            }
            else 
            {
                entries.last_mut().unwrap().push(0);
            }
        }
    }

    let ogen = filter_entries(entries.clone(), 0, 1);
    let co2scrub = filter_entries(entries, 0, 0);

    println!("{}", ogen * co2scrub);

    Ok(())
}