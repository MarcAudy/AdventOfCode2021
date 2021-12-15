use std::collections::HashSet;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Location
{
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Fold
{
    index: i16,
    x_fold: bool
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day13_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut folds: Vec<Fold> = Vec::new();
    let mut dots: HashSet<Location> = HashSet::new();

    for l in buffered.lines()
    {       
        let line = l?;

        let loc_split: Vec<&str> = line.split(",").collect();
        if loc_split.len() == 2
        {
            dots.insert(Location{x:loc_split[0].parse::<i16>().unwrap(), y:loc_split[1].parse::<i16>().unwrap()});
        }      
        else
        {
            let fold_split: Vec<&str> = line.split("=").collect();
            if fold_split.len() == 2
            {
                folds.push(Fold{index:fold_split[1].parse::<i16>().unwrap(),x_fold:fold_split[0].chars().last().unwrap() == 'x'});
            }
        }
    }

    for fold in folds
    {
        let mut new_dots: HashSet<Location> = HashSet::new();
        if fold.x_fold
        {
            for dot in dots
            {
                if dot.x > fold.index
                {
                    new_dots.insert(Location{x:fold.index-(dot.x-fold.index),y:dot.y});
                }
                else
                {
                    new_dots.insert(dot);
                }
            }
            let smallest_dot = new_dots.iter().map(|dot| dot.x).min().unwrap();
            if smallest_dot < 0
            {
                dots = new_dots.iter().map(|dot| Location{x:dot.x-smallest_dot,y:dot.y}).collect();
            }
            else
            {
                dots = new_dots;
            }
        }
        else
        {
            for dot in dots
            {
                if dot.y > fold.index
                {
                    new_dots.insert(Location{x:dot.x,y:fold.index-(dot.y-fold.index)});
                }
                else
                {
                    new_dots.insert(dot);
                }
            }
            let smallest_dot = new_dots.iter().map(|dot| dot.y).min().unwrap();
            if smallest_dot < 0
            {
                dots = new_dots.iter().map(|dot| Location{x:dot.x,y:dot.y-smallest_dot}).collect();
            }
            else
            {
                dots = new_dots;
            }
        }

        println!("{}", dots.len());
    }

    let max_x = dots.iter().map(|dot| dot.x).max().unwrap();
    let max_y = dots.iter().map(|dot| dot.y).max().unwrap();

    for y in 0..=max_y
    {
        for x in 0..=max_x
        {
            print!("{}", if dots.contains(&Location{x,y}) { '#' } else { '.'});
        }
        println!();
    }

    Ok(())
}