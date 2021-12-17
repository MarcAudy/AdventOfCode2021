use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day17_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input); 
    
    let r = Regex::new(r"target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)").unwrap();

    for l in buffered.lines()
    {
        let line = l?;
        let caps = r.captures(&line).unwrap();

        let xrange = caps[1].parse::<i32>().unwrap()..caps[2].parse::<i32>().unwrap()+1;
        let yrange = caps[3].parse::<i32>().unwrap()..caps[4].parse::<i32>().unwrap()+1;

        let mut valid_y_vals: Vec<(i32,i32)> = Vec::new();

        for y in yrange.start..-yrange.start
        {
            let mut yvel = y;
            let mut ypos = 0;
            let mut ysteps = 0;
            while ypos >= yrange.start
            {
                ypos += yvel;
                ysteps += 1;
                if yrange.contains(&ypos)
                {
                    valid_y_vals.push((y, ysteps));
                }
                yvel -= 1;
            }
        }

        let mut valid_initial_vels: HashSet<(i32,i32)> = HashSet::new();

        for x in 1..xrange.end
        {
            let mut xvel = x;
            let mut xpos = 0;
            let mut xsteps = 0;
            while xvel > 0
            {
                xpos += xvel;
                xsteps += 1;
                xvel -= 1;
                if xrange.contains(&xpos)
                {
                    let ys_for_x: Vec<&(i32,i32)> = valid_y_vals.iter().filter(|e| e.1 == xsteps || (xvel==0 && e.1 > xsteps)).into_iter().collect();
                    valid_initial_vels.extend::<HashSet<(i32,i32)>>(ys_for_x.iter().map(|y| (x,y.0)).collect());
                }
                else if xpos > xrange.end
                {
                    break;
                }
            }
        }

        println!("{}", valid_initial_vels.len());
    }


    Ok(())
}