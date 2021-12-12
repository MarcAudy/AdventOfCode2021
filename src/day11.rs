use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
struct Location
{
    x: i8,
    y: i8,
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day11_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut octopus: Vec<u8> = Vec::new();
    let cols: i8 = 10;
    let rows: i8 = 10;

    for l in buffered.lines()
    {       
        let line = l?;
        for c in line.chars()
        {
            octopus.push(String::from(c).parse::<u8>().unwrap());
        }
    }

    let get_index = | x: i8, y: i8 | (y * cols + x) as usize;

    let mut to_consider_init: HashMap<Location, u8> = HashMap::new();

    for x in 0..cols
    {
        for y in 0..rows
        {
            to_consider_init.insert(Location{x,y},1);
        }
    }    
    
    let mut loop_count = 0;
    let mut flashes = 0;

    loop
    {
        let mut to_consider: HashMap<Location, u8> = to_consider_init.clone();
        let mut flashed: Vec<Location> = Vec::new();

        while !to_consider.is_empty()
        {
            let (&cur_location, &inc) = to_consider.iter().next().unwrap();
            to_consider.remove(&cur_location);
            let cur_index = get_index(cur_location.x, cur_location.y);
            octopus[cur_index] += inc;
            if octopus[cur_index] >= 10
            {
                flashed.push(cur_location);

                for x_offset in (if cur_location.x > 0 { -1 } else { 0 })..=(if cur_location.x < cols-1 { 1 } else { 0 })
                {
                    for y_offset in (if cur_location.y > 0 { -1 } else { 0 })..=(if cur_location.y < cols-1 { 1 } else { 0 })
                    {
                        if !(x_offset == 0 && y_offset == 0)
                        {
                            let next_x = cur_location.x + x_offset;
                            let next_y = cur_location.y + y_offset;
                            let next_index = get_index(next_x, next_y);
                            if octopus[next_index] < 10
                            {
                                let next_loc = Location{x:next_x, y:next_y};
                                let inc = to_consider.entry(next_loc).or_insert(0);
                                *inc += 1
                            }
                        }
                    }               
                }
            }
        }

        if loop_count < 100
        {
            flashes += flashed.len();
        }

        loop_count = loop_count + 1;

        if flashed.len() == 100
        {
            break;
        }

        for loc in flashed
        {
            octopus[get_index(loc.x, loc.y)] = 0;
        }
    }

    println!("Flashes in first 100 iters: {}", flashes);
    println!("Synchronous flash on iter: {}", loop_count);

    Ok(())
}