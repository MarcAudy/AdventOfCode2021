use itertools::Itertools;
use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn distance_squared_between(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32
{
    (b.0 - a.0).pow(2) + (b.1 - a.1).pow(2) + (b.2 - a.2).pow(2)
}

fn manhattan_between(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32
{
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

fn main() -> Result<(), Error> 
{
    let rotations: Vec<Box<dyn Fn((i32,i32,i32))->(i32,i32,i32)>> = vec![
        Box::new(move |p: (i32,i32,i32)| { (p.0,p.1,p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (p.0,-p.2,p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (p.0,-p.1,-p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (p.0,p.2,-p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.0,-p.1,p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.0,p.2,p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.0,p.1,-p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.0,-p.2,-p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (p.1,p.2,p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (p.1,-p.0,p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (p.1,-p.2,-p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (p.1,p.0,-p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.1,-p.2,p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.1,p.0,p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.1,p.2,-p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.1,-p.0,-p.2) }),
        Box::new(move |p: (i32,i32,i32)| { (p.2,p.0,p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (p.2,-p.1,p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (p.2,-p.0,-p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (p.2,p.1,-p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.2,-p.0,p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.2,p.1,p.0) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.2,p.0,-p.1) }),
        Box::new(move |p: (i32,i32,i32)| { (-p.2,-p.1,-p.0) }),
    ];   

    let path = "d:\\AdventOfCode2021\\src\\day19_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input); 
    
    let mut inputs: Vec<Vec<(i32,i32,i32)>> = Vec::new();

    for l in buffered.lines()
    {
        let line = l?;

        if line.len() == 0
        {
            continue;
        }
        else if line.starts_with("--")
        {
            inputs.push(Vec::new());
        }
        else
        {
            let xyz: Vec<&str> = line.split(',').collect();
            inputs.last_mut().unwrap().push((
                xyz[0].parse::<i32>().unwrap(),
                xyz[1].parse::<i32>().unwrap(),
                xyz[2].parse::<i32>().unwrap()
            ));
        }
    }

    let distances: Vec<Vec<i32>> = inputs.iter().map(|input| input.iter().combinations(2).map(|combos| distance_squared_between(*combos[0],*combos[1]) ).collect()).collect();
    let mut per_node_distances: Vec<Vec<HashMap<usize,i32>>> = Vec::new();

    for input in &inputs
    {
        let mut node_distances = Vec::new();
        for i in 0..input.len()
        {
            let mut node_distance_map = HashMap::new();
            for j in 0..input.len()
            {
                if i != j
                {
                    node_distance_map.insert(j, distance_squared_between(input[i],input[j]));
                }
            }
            node_distances.push(node_distance_map);
        }
        per_node_distances.push(node_distances);
    }

    let target_count = 11*12/2;

    let mut to_consider: Vec<usize> = vec![0];
    let mut in_zero_space: HashSet<usize> = HashSet::from_iter(to_consider.iter().cloned());
    let mut scanner_pos: Vec<(i32,i32,i32)> = vec![(0,0,0);inputs.len()];

    while to_consider.len() > 0
    {
        let considering = to_consider.pop().unwrap();

        for i in 0..distances.len()
        {
            if !in_zero_space.contains(&i)
            {
                let match_count = distances[considering].iter().filter(|d| distances[i].contains(d)).count();
                if match_count >= target_count
                {
                    in_zero_space.insert(i);

                    let considering_nodes = &per_node_distances[considering];
                    let matched_nodes = &per_node_distances[i];
                    let mut unmatched_nodes: Vec<usize> = (0..matched_nodes.len()).collect();

                    let mut matching_nodes: Vec<(usize,usize)> = Vec::new();
                    for cn_index in 0..considering_nodes.len()
                    {
                        for um_index in 0..unmatched_nodes.len()
                        {
                            let mn_index = unmatched_nodes[um_index];
                            if considering_nodes[cn_index].iter().filter(|d| matched_nodes[mn_index].values().any(|&v| v == *d.1)).count() == 11
                            {
                                matching_nodes.push((cn_index, mn_index));
                                unmatched_nodes.remove(um_index);
                                break;
                            }
                        }
                        if matching_nodes.len() == 12
                        {
                            break;
                        }
                    }
                    
                    let point_pair_0 = (inputs[considering][matching_nodes[0].0],inputs[i][matching_nodes[0].1]);
                    let point_pair_1 = (inputs[considering][matching_nodes[1].0],inputs[i][matching_nodes[1].1]);

                    for rotation in &rotations
                    {
                        let pp01 = rotation(point_pair_0.1);
                        let pp11 = rotation(point_pair_1.1);

                        let point_offset_0 = (point_pair_0.0.0 - pp01.0, point_pair_0.0.1 - pp01.1, point_pair_0.0.2 - pp01.2);
                        let point_offset_1 = (point_pair_1.0.0 - pp11.0, point_pair_1.0.1 - pp11.1, point_pair_1.0.2 - pp11.2);

                        if point_offset_0 == point_offset_1
                        {
                            inputs[i] = inputs[i].iter().map(|p| rotation(*p)).collect();
                            scanner_pos[i] = point_offset_0;
                            break;
                        }
                    }

                    for mn in matching_nodes
                    {
                        inputs[i][mn.1] = inputs[considering][mn.0];
                    }
                    for um in unmatched_nodes
                    {
                        inputs[i][um].0 += scanner_pos[i].0;
                        inputs[i][um].1 += scanner_pos[i].1;
                        inputs[i][um].2 += scanner_pos[i].2;
                    }

                    if in_zero_space.len() == distances.len()
                    {
                        to_consider.clear();
                        break;
                    }
                    to_consider.push(i);
                }    
            }
        }
    }

    let mut all_beacons: Vec<(i32,i32,i32)> = inputs.into_iter().flatten().collect();
    all_beacons.sort();
    all_beacons.dedup();

    println!("Beacon count: {}", all_beacons.len());

    let manhattans: Vec<i32> = scanner_pos.iter().combinations(2).map(|combos| manhattan_between(*combos[0],*combos[1]) ).collect();

    println!("Largest manhattan: {}", manhattans.iter().max().unwrap());

    Ok(())

}