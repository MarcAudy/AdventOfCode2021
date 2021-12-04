use std::fs::File;
use std::io::{BufReader, BufRead, Error};

const PART1: bool = false;

struct Space
{
    number: u8,
    marked: bool,
}

fn mark_board(board: &mut[Vec<Space>], called_num: u8)
{
    for x in 0..5
    {
        for y in 0..5
        {
            if board[x][y].number == called_num
            {
                board[x][y].marked = true;
            }
        }
    }
}

fn score_board(board: &[Vec<Space>]) -> u64
{
    let mut result: u64 = 0;
    for x in 0..5
    {
        for y in 0..5
        {
            if !board[x][y].marked
            {
                result = result + board[x][y].number as u64;
            }
        }
    }

    return result;
}

fn is_winner(board: &[Vec<Space>]) -> bool
{
    for x in 0..5
    {
        if board[x].iter().filter(|&space| space.marked ).count() == 5
        {
            return true;
        }
    }
    for y in 0..5
    {
        let mut count = 0;
        for x in 0..5
        {
            if board[x][y].marked
            {
                count = count + 1;
            }
        }
        if count == 5
        {
            return true;
        }
    }

    return false;
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day4_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
  
    let mut lines = buffered.lines();

    let call_order : Vec<u8> = lines.next().unwrap()?.split(",").map(|x| x.parse::<u8>().unwrap()).collect();

    let mut boards : Vec<Vec<Vec<Space>>> = Vec::new();

    while !lines.next().is_none() // consume the blank
    {
        boards.push(Vec::new());
        for _ in 0..5
        {
            boards.last_mut().unwrap().push(lines.next().unwrap()?.split_whitespace().map(|x| Space { number: x.parse::<u8>().unwrap(), marked: false }).collect());
        }
    }

    if PART1
    {
        for called_num in call_order
        {
            for b in boards.iter_mut()
            {
                mark_board(b.as_mut_slice(), called_num);

                if is_winner(&b)
                {
                    let score = score_board(&b);
                    println!("{}", score * called_num as u64);
                    return Ok(());
                }
            }
        }
    }
    else
    {
        for called_num in call_order
        {
            for b in boards.iter_mut()
            {
                mark_board(b.as_mut_slice(), called_num);
            }

            if boards.len() == 1
            {
                if is_winner(&boards[0])
                {
                    let score = score_board(&boards[0]);
                    println!("{} {}", score , called_num as u64);
                    println!("{}", score * called_num as u64);
                    break;
                }
            }
            else
            {
                boards.retain(|b| !is_winner(b));
            }
        }
    }

    Ok(())
}