use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Location
{
    x: usize,
    y: usize,
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day25_input.ini";
    let input = File::open(path)?;
    let buffered = BufReader::new(input); 
    
    let mut east_cukes: HashSet<Location> = HashSet::new();
    let mut south_cukes: HashSet<Location> = HashSet::new();

    let mut cols: usize = 0;
    let mut rows: usize = 0;

    for l in buffered.lines()
    {
        let line = l?;
        let mut x: usize = 0;
        for c in line.chars()
        {
            match c
            {
                '.' => { },
                '>' => { east_cukes.insert(Location{x,y:rows}); },
                'v' => { south_cukes.insert(Location{x,y:rows}); },
                _ => std::panic!("unexpected input")
            }
            x += 1;
        }

        cols = x;
        rows = rows + 1;    
    }

    let mut steps = 0;

    loop
    {
        steps += 1;

        let mut moved = false;
        let mut new_east_cukes: HashSet<Location> = HashSet::new();

        for cuke in &east_cukes
        {
            let new_east_loc = Location { x: if cuke.x == cols-1 { 0 } else { cuke.x + 1 }, y: cuke.y };
            if !east_cukes.contains(&new_east_loc) && !south_cukes.contains(&new_east_loc)
            {
                new_east_cukes.insert(new_east_loc);
                moved = true;
            }
            else
            {
                new_east_cukes.insert(*cuke);
            }
        }
        east_cukes = new_east_cukes;

        let mut new_south_cukes: HashSet<Location> = HashSet::new();

        for cuke in &south_cukes
        {
            let new_south_loc = Location { x: cuke.x, y: if cuke.y == rows-1 { 0 } else { cuke.y + 1 } };
            if !east_cukes.contains(&new_south_loc) && !south_cukes.contains(&new_south_loc)
            {
                new_south_cukes.insert(new_south_loc);
                moved = true;
            }
            else
            {
                new_south_cukes.insert(*cuke);
            }
        }
        south_cukes = new_south_cukes;

        if moved == false
        {
            break;
        }
    }

    println!("{}", steps);

    Ok(())
}
