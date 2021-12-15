use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day14_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();

    let mut polymer: Vec<char> = lines.next().unwrap()?.chars().collect();
    let mut mappings: HashMap<(char,char), char> = HashMap::new();

    lines.next(); // consume blank line

    for l in lines
    {       
        let line = l?;
        let mapping: Vec<&str> = line.split(" -> ").collect();
        let mapping_key: Vec<char> = mapping[0].to_string().chars().collect();
        mappings.insert((mapping_key[0],mapping_key[1]), mapping[1].to_string().chars().collect::<Vec<char>>()[0]);
    }
    
    for _ in 0..10
    {
        let mut new_polymer: Vec<char> = Vec::new();

        for index in 1..polymer.len()
        {
            new_polymer.push(polymer[index-1]);
            new_polymer.push(*mappings.get(&(polymer[index-1],polymer[index])).unwrap());
        }
        new_polymer.push(*polymer.last().unwrap());

        polymer = new_polymer;
    }

    let mut unique_element_counts: HashMap<char, usize> = HashMap::new();
    for p in polymer
    {
        *unique_element_counts.entry(p).or_insert(0) += 1;
    }

    println!("{}", unique_element_counts.iter().map(|x| x.1).max().unwrap() - unique_element_counts.iter().map(|x| x.1).min().unwrap());

    Ok(())
}