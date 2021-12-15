use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day14_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();

    let polymer: Vec<char> = lines.next().unwrap()?.chars().collect();
    let mut mappings: HashMap<(char,char), char> = HashMap::new();

    lines.next(); // consume blank line

    for l in lines
    {       
        let line = l?;
        let mapping: Vec<&str> = line.split(" -> ").collect();
        let mapping_key: Vec<char> = mapping[0].to_string().chars().collect();
        mappings.insert((mapping_key[0],mapping_key[1]), mapping[1].to_string().chars().collect::<Vec<char>>()[0]);
    }
    
    let mut pair_counts: HashMap<(char,char), i64> = HashMap::new();

    for index in 1..polymer.len()
    {
        *pair_counts.entry((polymer[index-1], polymer[index])).or_insert(0) += 1;
    }

    for _ in 0..40
    {
        let mut new_pair_counts: HashMap<(char,char), i64> = HashMap::new();

        for pair in pair_counts
        {
            let middle_char = *mappings.get(&pair.0).unwrap();
            *new_pair_counts.entry((pair.0.0, middle_char)).or_insert(0) += pair.1;
            *new_pair_counts.entry((middle_char, pair.0.1)).or_insert(0) += pair.1;
        }

        pair_counts = new_pair_counts;
    }

    let mut unique_element_counts: HashMap<char, i64> = HashMap::new();
    for pair in pair_counts
    {
        *unique_element_counts.entry(pair.0.0).or_insert(0) += pair.1;
    }
    *unique_element_counts.entry(*polymer.last().unwrap()).or_insert(0) += 1;

    println!("{:?}", unique_element_counts);

    println!("{}", unique_element_counts.iter().map(|x| x.1).max().unwrap() - unique_element_counts.iter().map(|x| x.1).min().unwrap());    

    Ok(())
}