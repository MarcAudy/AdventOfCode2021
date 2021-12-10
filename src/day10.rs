use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day10_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut total_corrupted_score: i32 = 0;
    let mut autocomplete_scores: Vec<i64> = Vec::new();

    for l in buffered.lines()
    {       
        let mut chunks: Vec<char> = Vec::new();

        let mut process_char = |c| -> Result<i32, Error>
        {
            match c 
            {
                '(' | '<' | '[' | '{' => { chunks.push(c); return Ok(0) },
                ')' => if *chunks.last().unwrap() == '(' { chunks.pop(); return Ok(0); } else { return Ok(3); },
                ']' => if *chunks.last().unwrap() == '[' { chunks.pop(); return Ok(0); } else { return Ok(57); },
                '}' => if *chunks.last().unwrap() == '{' { chunks.pop(); return Ok(0); } else { return Ok(1197); },
                '>' => if *chunks.last().unwrap() == '<' { chunks.pop(); return Ok(0); } else { return Ok(25137); },
                _ => return Err(Error::new(ErrorKind::InvalidData, format!("Unexpected character '{}'", c)))
            }
        };

        let mut corrupted_score = 0;

        let line = l?;
        for c in line.chars()
        {
            let result = process_char(c).unwrap();
            if result > 0
            {
                corrupted_score = corrupted_score + result;
                break;
            }
        }
        total_corrupted_score = total_corrupted_score + corrupted_score;

        if corrupted_score == 0
        {
            let mut autocomplete_score = 0;
            chunks.reverse();
            for chunk in chunks
            {
                autocomplete_score = autocomplete_score * 5;
                autocomplete_score = autocomplete_score + 
                    match chunk
                    {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => 0
                    };
            }

            autocomplete_scores.push(autocomplete_score);
        }
    }

    autocomplete_scores.sort();

    println!("Part1: {}", total_corrupted_score);
    println!("Part2: {}", autocomplete_scores[autocomplete_scores.len()/2]);

    Ok(())
}