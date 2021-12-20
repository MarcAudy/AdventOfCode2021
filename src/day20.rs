use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PART1: bool = false;
const ITER_COUNT: u8 = if PART1 { 2 } else { 50 };

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day20_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input); 
    
    let mut img_enhance_algo: Vec<u8> = Vec::new();
    let mut input_image: Vec<u8> = Vec::new();

    let mut rows: i32 = 0;

    for l in buffered.lines()
    {
        let line = l?;
        if img_enhance_algo.len() == 0
        {
            img_enhance_algo = line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect();
        }
        else if line.len() > 0
        {
            input_image.extend::<Vec<u8>>(line.chars().map(|c| if c == '.' { 0 } else { 1 }).collect());
            rows += 1;
        }
    }
    
    let mut cols = input_image.len() as i32 / rows;

    let get_pixel = | image: &Vec<u8>, x: i32, y: i32, rows: i32, cols: i32, iter: u8 | -> u8
    {
        if x < 0 || y < 0 || x > cols - 1 || y > rows - 1
        { 
            return if img_enhance_algo[0] == 0 || iter % 2 == 0 { 0 } else { 1 }
        }
        return image[(y * cols + x) as usize];
    };

    let get_value = | stream: &Vec<u8> | -> usize
    {
        let mut value = 0;
        for n in stream
        {
            value <<= 1;
            value += *n as i32;
        }

        return value as usize;
    };

    let determine_pixel = | image: &Vec<u8>, x: i32, y: i32, rows: i32, cols: i32, iter: u8 | -> u8
    {
        let mut stream: Vec<u8> = Vec::new();
        for y_offset in -1..=1
        {
            for x_offset in -1..=1
            {
                stream.push(get_pixel(&image, x + x_offset, y + y_offset, rows, cols, iter));
            }
        }
        return img_enhance_algo[get_value(&stream)];
    };

    for iter_count in 0..ITER_COUNT
    {
        let mut new_image: Vec<u8> = Vec::new();
        for y in -1..=rows
        {
            for x in -1..=cols
            {
                new_image.push(determine_pixel(&input_image, x, y, rows, cols, iter_count));
            }
        }
        cols += 2;
        rows += 2;
        input_image = new_image;
    }

    input_image.retain(|n| *n == 1);
    println!("{}", input_image.len());

    Ok(())

}