use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PART1: bool = false;

struct VentLine
{
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day5_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
  
    let r = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let mut vent_lines: Vec<VentLine> = Vec::new();

    // let mut cols: i32 = 0;
    // let mut rows: i32 = 0;

    for l in buffered.lines()
    {
        let line = l?;
        let caps = r.captures(&line).unwrap();
        let vl = VentLine {
            start_x: caps[1].parse::<i32>().unwrap(),
            start_y: caps[2].parse::<i32>().unwrap(),
            end_x: caps[3].parse::<i32>().unwrap(),
            end_y: caps[4].parse::<i32>().unwrap()
        };

        // cols = cmp::max(cols, cmp::max(vl.start_x, vl.end_x));
        // rows = cmp::max(rows, cmp::max(vl.start_y, vl.end_y));

        vent_lines.push(vl);
    }

    // cols = cols + 1;
    // rows = rows + 1;

    // This is probably idiomatic, but has to be silly expensive
    // If efficiency mattered better to just collect as the lines are read in
    let cols: i32 = vent_lines.iter().map(|vl| vl.start_x ).chain(vent_lines.iter().map(|vl| vl.end_x )).max().unwrap() + 1;
    let rows: i32 = vent_lines.iter().map(|vl| vl.start_y ).chain(vent_lines.iter().map(|vl| vl.end_y )).max().unwrap() + 1;

    let get_floor_index = | x: i32, y: i32 | (y * cols + x) as usize;

    let mut ocean_floor: Vec<u8> = Vec::new();
    for _ in 0..rows*cols
    {
        ocean_floor.push(0);
    }

    for vl in vent_lines
    {
        if vl.start_y == vl.end_y
        {
            for x in cmp::min(vl.start_x,vl.end_x)..cmp::max(vl.start_x,vl.end_x)+1
            {
                let floor_index = get_floor_index(x,vl.start_y);
                ocean_floor[floor_index] = ocean_floor[floor_index] + 1;
            }
        }
        else if vl.start_x == vl.end_x
        {
            for y in cmp::min(vl.start_y,vl.end_y)..cmp::max(vl.start_y,vl.end_y)+1
            {
                let floor_index = get_floor_index(vl.start_x,y);
                ocean_floor[floor_index] = ocean_floor[floor_index] + 1;
            }
        }
        else if !PART1
        {
            let mut x = vl.start_x;
            let mut y = vl.start_y;
            let x_inc = if vl.start_x > vl.end_x { -1 } else { 1 };
            let y_inc = if vl.start_y > vl.end_y { -1 } else { 1 };
            for _ in 0..(vl.start_x-vl.end_x).abs()+1
            {
                let floor_index = get_floor_index(x,y);
                ocean_floor[floor_index] = ocean_floor[floor_index] + 1;
                x = x + x_inc;
                y = y + y_inc;
            }
        }
    }

    ocean_floor.retain(|x| x > &1);

    println!("{}", ocean_floor.len());

    Ok(())
}