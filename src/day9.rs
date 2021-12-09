use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(PartialEq, Eq, Copy, Clone)]
struct Location
{
    x: usize,
    y: usize,
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day9_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut floor: Vec<u8> = Vec::new();
    let mut cols: usize = 0;
    let mut rows: usize = 0;

    for l in buffered.lines()
    {       
        let line = l?;
        for c in line.chars()
        {
            floor.push(String::from(c).parse::<u8>().unwrap());
        }

        cols = line.len();
        rows = rows + 1;
    }

    let get_height = | x: usize, y: usize | floor[y * cols + x] as usize;
    
    let mut risk: usize = 0;
    let mut basins: [usize; 3] = [0; 3];

    for x in 0..cols
    {
        for y in 0..rows
        {
            let this_height = get_height(x,y);
            if    (x == 0 || this_height < get_height(x-1,y))
               && (x == cols-1 || this_height < get_height(x+1,y))
               && (y == 0 || this_height < get_height(x,y-1))
               && (y == rows-1 || this_height < get_height(x,y+1))
            {
                risk = risk + this_height + 1; 

                let mut basin: Vec<Location> = Vec::new();
                
                basin.push(Location{x,y});
                let mut index = 0;
                while index < basin.len()
                {
                    let loc = basin[index];
                    if loc.x > 0 && get_height(loc.x-1,loc.y) < 9 && !basin.iter().any(|l| *l == Location{x:loc.x-1,y:loc.y})
                    {
                        basin.push(Location{x:loc.x-1,y:loc.y});
                    }
                    if loc.x < cols-1 && get_height(loc.x+1,loc.y) < 9 && !basin.iter().any(|l| *l == Location{x:loc.x+1,y:loc.y})
                    {
                        basin.push(Location{x:loc.x+1,y:loc.y});
                    }
                    if loc.y > 0 && get_height(loc.x,loc.y-1) < 9 && !basin.iter().any(|l| *l == Location{x:loc.x,y:loc.y-1})
                    {
                        basin.push(Location{x:loc.x,y:loc.y-1});
                    }
                    if loc.y < rows-1 && get_height(loc.x,loc.y+1) < 9 && !basin.iter().any(|l| *l == Location{x:loc.x,y:loc.y+1})
                    {
                        basin.push(Location{x:loc.x,y:loc.y+1});
                    }
                    index = index + 1;
                }

                for i in (0..3).rev()
                {
                    if basin.len() > basins[i]
                    {
                        for j in 0..i
                        {
                            basins[j] = basins[j+1];
                        }
                        basins[i] = basin.len();
                        break;
                    }
                }
            }
        }
    }

    println!("Part1: {}", risk);
    println!("Part2: {}", basins[0] * basins[1] * basins[2]);

    Ok(())
}