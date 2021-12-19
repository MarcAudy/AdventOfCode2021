use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead, Error, ErrorKind};

const PART1: bool = false;

#[derive(Clone)]
enum Element
{
    Value(i32),
    Pair(Box<Pair>)
}

fn get_element_value(e: &Element) -> Result<i32, Error>
{
    match e
    {
        Element::Value(val) => Ok(*val),
        _ => Err(Error::new(ErrorKind::InvalidData,""))
    }
}

impl fmt::Debug for Element
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        match self
        {
            Element::Value(val) => { write!(f, "{}", val) },
            Element::Pair(pair) => { write!(f, "{:?}", pair) }
        }
    }
}

#[derive(Clone)]
struct Pair
{
    left: Element,
    right: Element
}

impl fmt::Debug for Pair
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
    {
        write!(f, "[{:?},{:?}]", self.left, self.right)
    }
}
impl Pair
{
    fn new(input: &str) -> Pair
    {
        let left;
        let right;

        if input.chars().nth(0) == Some('[')
        {
            let mut open_count = 0;
            let mut index = 0;
            for c in input.chars()
            {
                match c
                {
                    '[' => open_count += 1,
                    ']' => open_count -= 1,
                    _ => {}
                }

                index += 1;

                if open_count == 0
                {
                    break;
                }
            }

            assert_eq!(input.chars().nth(index).unwrap(), ',');

            left = Element::Pair(Box::new(Pair::new(&input[1..index-1])));

            if input.chars().nth(index+1) == Some('[')
            {
                right = Element::Pair(Box::new(Pair::new(&input[index+2..input.len()-1])));
            }
            else
            {
                right = Element::Value(input[index+1..].parse::<i32>().unwrap());
            }
        }
        else
        {
            let split = input.split_at(input.chars().position(|c| c == ',').unwrap());
            left = Element::Value(split.0.parse::<i32>().unwrap());

            if split.1.chars().nth(1) == Some('[')
            {
                right = Element::Pair(Box::new(Pair::new(&split.1[2..split.1.len()-1])));
            }
            else
            {
                right = Element::Value(split.1[1..].parse::<i32>().unwrap());
            }
        }

        Pair { left, right }
    }

    fn explode(&mut self, depth: u8) -> Result<(i32,i32),Error>
    {
        if depth == 4
        {
            let left_val = get_element_value(&self.left).unwrap();
            let right_val = get_element_value(&self.right).unwrap();

            return Ok((left_val,right_val));
        }

        match &self.left
        {
            Element::Pair(pair) =>
            {
                let mut pair_to_explode = pair.clone();
                match pair_to_explode.explode(depth+1)
                {
                    Ok(result) => 
                    {
                        if depth == 3
                        {
                            self.left = Element::Value(0);
                        }
                        else
                        {
                            self.left = Element::Pair(pair_to_explode);                       
                        }

                        if result.1 > 0
                        {
                            match &self.right
                            {
                                Element::Value(val) => self.right = Element::Value(result.1+val),
                                Element::Pair(pair) => {
                                    let mut pair_copy = pair.clone();
                                    pair_copy.add_value_to_leftmost(result.1);
                                    self.right = Element::Pair(pair_copy);
                                }
                            }
                            return Ok((result.0,0));
                        }

                        return Ok(result);
                    },
                    Err(_) => {}
                }
            },
            _ => {},
        }

        match &self.right
        {
            Element::Pair(pair) =>
            {
                let mut pair_to_explode = pair.clone();
                match pair_to_explode.explode(depth+1)
                {
                    Ok(result) => 
                    {
                        if depth == 3
                        {
                            self.right = Element::Value(0);
                        }
                        else
                        {
                            self.right = Element::Pair(pair_to_explode);
                        }

                        if result.0 > 0
                        {
                            match &self.left
                            {
                                Element::Value(val) => self.left = Element::Value(result.0+val),
                                Element::Pair(pair) => {
                                    let mut pair_copy = pair.clone();
                                    pair_copy.add_value_to_rightmost(result.0);
                                    self.left = Element::Pair(pair_copy);
                                }
                            }
                            return Ok((0,result.1));
                        }
                        
                        return Ok(result);
                    },
                    Err(_) => {}
                }
            },
            _ => {},
        }

        Err(Error::new(ErrorKind::NotFound,""))
    }

    fn split(&mut self) -> Result<(),Error>
    {
        match &self.left
        {
            Element::Value(val) => { 
                if val >= &10
                {
                    let left = Element::Value((*val as f32 / 2.0).floor() as i32);
                    let right = Element::Value((*val as f32 / 2.0).ceil() as i32);
                    self.left = Element::Pair(Box::new(Pair { left, right }));
                    return Ok(());
                }
            },
            Element::Pair(pair) => {
                let mut pair_to_split = pair.clone();
                match pair_to_split.split()
                {
                    Ok(_) => { 
                        self.left = Element::Pair(pair_to_split);
                        return Ok(()); 
                    },
                    Err(_) => {}
                }
            }
        };
        match &self.right
        {
            Element::Value(val) => { 
                if val >= &10
                {
                    let left = Element::Value((*val as f32 / 2.0).floor() as i32);
                    let right = Element::Value((*val as f32 / 2.0).ceil() as i32);
                    self.right = Element::Pair(Box::new(Pair { left, right }));
                    return Ok(());
                }
            },
            Element::Pair(pair) => {
                let mut pair_to_split = pair.clone();
                match pair_to_split.split()
                {
                    Ok(_) => { 
                        self.right = Element::Pair(pair_to_split);
                        return Ok(()); 
                    },
                    Err(_) => {}
                }
            }        };

        Err(Error::new(ErrorKind::NotFound,""))
    }

    fn add_value_to_leftmost(&mut self, value: i32)
    {
        match &self.left
        {
            Element::Value(val) => self.left = Element::Value(val+value),
            Element::Pair(pair) => {
                let mut pair_copy = pair.clone();
                pair_copy.add_value_to_leftmost(value);
                self.left = Element::Pair(pair_copy);
            }
        }
    }

    fn add_value_to_rightmost(&mut self, value: i32)
    {
        match &self.right
        {
            Element::Value(val) => self.right = Element::Value(val+value),
            Element::Pair(pair) => {
                let mut pair_copy = pair.clone();
                pair_copy.add_value_to_rightmost(value);
                self.right = Element::Pair(pair_copy);
            }
        }
    }

    fn get_magnitude(&self) -> i32
    {
        let left_mag = 3 * match &self.left
        {
            Element::Value(val) => *val,
            Element::Pair(pair) => pair.get_magnitude()
        };
        let right_mag = 2 * match &self.right
        {
            Element::Value(val) => *val,
            Element::Pair(pair) => pair.get_magnitude()
        };
        return left_mag + right_mag;
    }
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day18_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input); 
    
    let mut inputs: Vec<Pair> = Vec::new();

    for l in buffered.lines()
    {
        let line = l?;
        inputs.push(Pair::new(&line[1..line.len()-1]));
    }

    if PART1
    {
        let mut sum = inputs[0].clone();

        for index in 1..inputs.len()
        {
            let mut new_pair = Pair{ left: Element::Pair(Box::new(sum)), right: Element::Pair(Box::new(inputs[index].clone())) };

            loop
            {
                match new_pair.explode(0)
                {
                    Ok(_) => {},
                    Err(_) => match new_pair.split()
                    {
                        Ok(_) => {},
                        Err(_) => break
                    }
                }
            }

            sum = new_pair;
        }

        println!("{}", sum.get_magnitude());
    }
    else
    {
        let mut max_mag = 0;

        let add_two_pairs = |a: &Pair, b: &Pair| -> i32
        {
            let mut new_pair = Pair{ left: Element::Pair(Box::new(a.clone())), right: Element::Pair(Box::new(b.clone())) };
    
            loop
            {
                match new_pair.explode(0)
                {
                    Ok(_) => {},
                    Err(_) => match new_pair.split()
                    {
                        Ok(_) => {},
                        Err(_) => break
                    }
                }
            }
    
            return new_pair.get_magnitude()
        };
    
        for i in 0..inputs.len()-1
        {
            for j in i+1..inputs.len()
            {
                let mut mag = add_two_pairs(&inputs[i],&inputs[j]);
                if mag > max_mag
                {
                    max_mag = mag;
                }      
        
                mag = add_two_pairs(&inputs[j],&inputs[i]);
                if mag > max_mag
                {
                    max_mag = mag;
                }      
    
            }
        }
    
        println!("{}", max_mag);        
    }

    Ok(())
}