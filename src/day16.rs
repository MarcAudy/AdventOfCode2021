use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn get_value(stream: &[i8]) -> usize
{
    let mut value = 0;
    for n in stream
    {
        value <<= 1;
        value += *n as i32;
    }

    return value as usize;
}

fn parse_stream(stream: &[i8]) -> (usize, usize, usize)
{
    let mut consumed = 6;

    let mut packet_value = 0;

    let packet_version = get_value(&stream[0..3]);
    let packet_type = get_value(&stream[3..6]);

    let mut packet_versions = packet_version;

    if packet_type == 4
    {
        loop
        {
            let last_digit = get_value(&[stream[consumed]]) == 0;
            packet_value *= 16;
            packet_value += get_value(&stream[consumed+1..consumed+5]);

            consumed += 5;

            if last_digit
            {
                break; 
            }
        }
    }
    else
    {
        let mut subpacket_values: Vec<usize> = Vec::new();
        let length_type = get_value(&[stream[6]]);
        if length_type == 0
        {
            let mut subpacket_length = get_value(&stream[7..22]);
            consumed = 22 + subpacket_length;

            let mut start_next_subpacket = 22;
            while subpacket_length > 0
            {
                let (subpacket_consumed, subpacket_versions, subpacket_value) = parse_stream(&stream[start_next_subpacket..]);
                packet_versions += subpacket_versions;
                start_next_subpacket += subpacket_consumed as usize;
                subpacket_length -= subpacket_consumed;
                subpacket_values.push(subpacket_value);
            }
        }
        else
        {
            let num_subpackets = get_value(&stream[7..18]);
            consumed = 18;

            for _ in 0..num_subpackets
            {
                let (subpacket_consumed, subpacket_versions, subpacket_value) = parse_stream(&stream[consumed..]);
                consumed += subpacket_consumed;
                packet_versions += subpacket_versions;
                subpacket_values.push(subpacket_value);
            }
        }

        packet_value = match packet_type
        {
            0 => { subpacket_values.iter().fold(0, |result, value| result + value) }
            1 => { subpacket_values.iter().fold(1, |result, value| result * value) }
            2 => { *subpacket_values.iter().min().unwrap() }
            3 => { *subpacket_values.iter().max().unwrap() }
            5 => { assert_eq!(subpacket_values.len(), 2); if subpacket_values[0] > subpacket_values[1] { 1 } else { 0 } }
            6 => { assert_eq!(subpacket_values.len(), 2); if subpacket_values[0] < subpacket_values[1] { 1 } else { 0 } }
            7 => { assert_eq!(subpacket_values.len(), 2); if subpacket_values[0] == subpacket_values[1] { 1 } else { 0 } }
            _ => { assert_eq!(true, false); 0 }
        };
    }

    return (consumed, packet_versions, packet_value);
}


fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day16_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let get_binary = |hex|
    {
        match hex
        {
            '0' => [0,0,0,0],
            '1' => [0,0,0,1],
            '2' => [0,0,1,0],
            '3' => [0,0,1,1],
            '4' => [0,1,0,0],
            '5' => [0,1,0,1],
            '6' => [0,1,1,0],
            '7' => [0,1,1,1],
            '8' => [1,0,0,0],
            '9' => [1,0,0,1],
            'A' => [1,0,1,0],
            'B' => [1,0,1,1],
            'C' => [1,1,0,0],
            'D' => [1,1,0,1],
            'E' => [1,1,1,0],
            'F' => [1,1,1,1],
            _ => [-1,-1,-1,-1]
        }
    };

    let mut binary_stream: Vec<i8> = Vec::new();

    for l in buffered.lines()
    {       
        let line = l?;
        for c in line.chars()
        {
            binary_stream.extend_from_slice(&get_binary(c));
        }
    }   

    let (_, packet_versions, packet_value) = parse_stream(binary_stream.as_slice());

    println!("Part1: {}", packet_versions);
    println!("Part2: {}", packet_value);

    Ok(())
}