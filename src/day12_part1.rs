use bimap::BiMap;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

struct CaveData
{
    caves: BiMap<String, u8>,
    big_caves: HashSet<u8>,
    small_caves: HashSet<u8>,
    cave_connections: HashMap<u8, Vec<u8>>,
    start_node: u8,
    end_node: u8
}

impl CaveData
{
    fn new() -> CaveData
    {
        CaveData { 
            caves: BiMap::new(),
            big_caves: HashSet::new(),
            small_caves: HashSet::new(),
            cave_connections: HashMap::new(),
            start_node: u8::MAX,
            end_node: u8::MAX
        }
    }

    fn add_data(&mut self, line: String)  
    {
        let cave_pair: Vec<String> = line.split("-").map(|x| x.to_string()).collect();

        let first_cave;
        let second_cave;

        match self.caves.insert_no_overwrite(cave_pair[0].clone(), self.caves.len() as u8)
        {
            Ok(_) => {
                first_cave = (self.caves.len() - 1) as u8;
                if cave_pair[0] == cave_pair[0].to_uppercase()
                {
                    self.big_caves.insert(first_cave);
                }
                else
                {
                    self.small_caves.insert(first_cave);
                    match cave_pair[0].as_ref()
                    {
                        "start" => { self.start_node = first_cave; }
                        "end"   => { self.end_node = first_cave; }
                        _ => {}
                    }
                }
            }
            Err(_) => { first_cave = *self.caves.get_by_left(&cave_pair[0]).unwrap(); }
        };
        match self.caves.insert_no_overwrite(cave_pair[1].clone(), self.caves.len() as u8)
        {
            Ok(_) => {
                second_cave = (self.caves.len() - 1) as u8;
                if cave_pair[1] == cave_pair[1].to_uppercase()
                {
                    self.big_caves.insert(second_cave);
                }
                else
                {
                    self.small_caves.insert(second_cave);
                }
                match cave_pair[1].as_ref()
                {
                    "start" => { self.start_node = second_cave; }
                    "end"   => { self.end_node = second_cave; }
                    _ => {}
                }
        }
            Err(_) => { second_cave = *self.caves.get_by_left(&cave_pair[1]).unwrap(); }
        };

        self.cave_connections.entry(first_cave).or_insert(Vec::new()).push(second_cave);
        self.cave_connections.entry(second_cave).or_insert(Vec::new()).push(first_cave);    
    }
}

fn get_paths_from(cur_node: u8, prev_visited_nodes: &Vec<u8>, cave_data: &CaveData) -> Vec<Vec<u8>>
{   
    let mut paths: Vec<Vec<u8>> = Vec::new();
    let visited_nodes = [prev_visited_nodes.as_slice(), &[cur_node]].concat();
    for connected_node in cave_data.cave_connections.get(&cur_node).unwrap()
    {
        if *connected_node == cave_data.end_node
        {
            paths.push([visited_nodes.as_slice(), &[*connected_node]].concat());
            continue;
        }
        else if cave_data.small_caves.contains(connected_node)
        {
            if prev_visited_nodes.contains(connected_node)
            {
                continue;
            }
        }
        paths.append(&mut get_paths_from(*connected_node, &visited_nodes, cave_data));
    }

    return paths;
}

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day12_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut cave_data = CaveData::new();

    for l in buffered.lines()
    {       
        cave_data.add_data(l?);  
    }

    let paths = get_paths_from(cave_data.start_node, &Vec::new(), &cave_data);

    println!("{}", paths.len());

    Ok(())
}