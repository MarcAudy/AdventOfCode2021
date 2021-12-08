use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day8_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Positions
    {
        Top,
        TopLeft,
        TopRight,
        Middle,
        BottomLeft,
        BottomRight,
        Bottom
    }

    let mut result = 0;

    for l in buffered.lines()
    {
        let line = l?;
        let parts = line.split(" | ").collect::<Vec<&str>>();
        let mut unique_digits = parts[0].split_whitespace().collect::<Vec<&str>>();
        unique_digits.sort_by(|a,b| a.len().cmp(&b.len()));
        let output_value = parts[1].split_whitespace().collect::<Vec<&str>>();

        let contained_by_all_fives = |x| unique_digits[3].contains(x) && unique_digits[4].contains(x) && unique_digits[5].contains(x);
        let contained_by_all_sixes = |x| unique_digits[6].contains(x) && unique_digits[7].contains(x) && unique_digits[8].contains(x);

        let mut position_map: HashMap<Positions, char> = HashMap::new();

        // Top is the element that is in the 7 that isn't in the 1 
        position_map.insert(Positions::Top, unique_digits[1].chars().filter(|x| !unique_digits[0].contains(*x)).collect::<Vec<char>>()[0]);
        // Top Right is the element is in the 1 that is not in all 6-component numbers
        position_map.insert(Positions::TopRight, unique_digits[0].chars().filter(|x| !contained_by_all_sixes(*x)).collect::<Vec<char>>()[0]);
        // Bottom Right is the other element in the 1 that is in all the 6-component numbers
        position_map.insert(Positions::BottomRight, unique_digits[0].chars().filter(|x| contained_by_all_sixes(*x)).collect::<Vec<char>>()[0]);
        // Middle is the element from the 4 that is not in the 1 and not in all the 6-component numbers
        position_map.insert(Positions::Middle, unique_digits[2].chars().filter(|x| !unique_digits[0].contains(*x) && !contained_by_all_sixes(*x)).collect::<Vec<char>>()[0]);
        // Top left is the remaining element from the 4
        position_map.insert(Positions::TopLeft, unique_digits[2].chars().filter(|x| !position_map.values().any(|p| p == x)).collect::<Vec<char>>()[0]);
        // Bottom is the remaining element that is in all 5-component numbers
        position_map.insert(Positions::Bottom, String::from("abcdefg").chars().filter(|x| !position_map.values().any(|p| p == x) && contained_by_all_fives(*x)).collect::<Vec<char>>()[0]);
        // Bottom left is the final element
        position_map.insert(Positions::BottomLeft, String::from("abcdefg").chars().filter(|x| !position_map.values().any(|p| p == x) ).collect::<Vec<char>>()[0]);
        
        let translate_digit = |digit: &str| -> Result<i32, Error> {
            if digit.len() == 6 && !digit.chars().any(|x| x == *position_map.get(&Positions::Middle).unwrap())           { return Ok(0); }
            else if digit.len() == 2                                                                                     { return Ok(1); }
            else if digit.len() == 5 && !digit.chars().any(|x| x == *position_map.get(&Positions::BottomRight).unwrap()) { return Ok(2); }
            else if digit.len() == 5 && !digit.chars().any(|x| x == *position_map.get(&Positions::TopLeft).unwrap())     { return Ok(3); }
            else if digit.len() == 4                                                                                     { return Ok(4); }
            else if digit.len() == 5 && !digit.chars().any(|x| x == *position_map.get(&Positions::TopRight).unwrap())    { return Ok(5); }
            else if digit.len() == 6 && !digit.chars().any(|x| x == *position_map.get(&Positions::TopRight).unwrap())    { return Ok(6); }
            else if digit.len() == 3                                                                                     { return Ok(7); }
            else if digit.len() == 7                                                                                     { return Ok(8); }
            else if digit.len() == 6 && !digit.chars().any(|x| x == *position_map.get(&Positions::BottomLeft).unwrap())  { return Ok(9); }
            else { return Err(Error::new(ErrorKind::InvalidData, format!("Unexpected digit '{}'", digit))); }
        };

        result = result
                 + translate_digit(output_value[0]).unwrap() * 1000
                 + translate_digit(output_value[1]).unwrap() * 100 
                 + translate_digit(output_value[2]).unwrap() * 10 
                 + translate_digit(output_value[3]).unwrap();
    }

    println!("{}", result);

    Ok(())
}