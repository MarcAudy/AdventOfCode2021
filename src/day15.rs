use petgraph::Graph;
use petgraph::algo::astar;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Pos
{
    x: usize,
    y: usize,
}

const PART1: bool = false;

fn main() -> Result<(), Error> 
{
    let path = "d:\\AdventOfCode2021\\src\\day15_input.ini";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut cavern: Vec<Vec<usize>> = Vec::new();

    for l in buffered.lines()
    {       
        let line = l?;

        let mut crow = Vec::new();
        for c in line.chars()
        {
            crow.push(String::from(c).parse::<usize>().unwrap());
        }
        cavern.push(crow);
    }
   
    let mut cols: usize = cavern.last().unwrap().len();
    let mut rows: usize = cavern.len();

    if !PART1
    {
        for tx in 1..5
        {
            for y in 0..rows
            {
                for x in 0..cols
                {
                    let risk = cavern[y][x];
                    let mut new_risk = risk + tx;
                    if new_risk > 9 { new_risk -= 9; }
                    cavern[y].push(new_risk);
                }
            }
        }
        let cavern_copy = cavern.clone();
        for ty in 1..5
        {
            cavern.append(&mut cavern_copy.clone());
            for y in 0..rows
            {
                for r in cavern[y+ty*rows].iter_mut()
                {
                    *r += ty;
                    if *r > 9
                    {
                        *r -= 9;
                    }
                }
            }
        }
    }

    cols = cavern.last().unwrap().len();
    rows = cavern.len();

    let mut graph = Graph::<Pos,usize>::new();
    let mut pos_node_map = HashMap::new();

    for y in 0..rows
    {
        for x in 0..cols
        {
            pos_node_map.insert(Pos{x,y}, graph.add_node(Pos{x,y}));
            if !PART1
            {
                for tx in 0..5
                {
                    for ty in 0..5
                    {
                        if tx != 0 || ty != 0
                        {
                            let new_pos = Pos{x:x+tx*cols,y:y+ty*rows};
                            pos_node_map.insert(new_pos, graph.add_node(new_pos));
                        }
                    }
                }
            }
        }
    }

    for y in 0..rows
    {
        for x in 0..cols
        {
            let xy_node = *pos_node_map.get(&Pos{x,y}).unwrap();
            let xy_cost = cavern[y][x];
            if x > 0
            {
                graph.add_edge(*pos_node_map.get(&Pos{x:x-1,y}).unwrap(),xy_node,xy_cost);
            }
            if x < cols - 1
            {
                graph.add_edge(*pos_node_map.get(&Pos{x:x+1,y}).unwrap(),xy_node,xy_cost);
            }
            if y > 0
            {
                graph.add_edge(*pos_node_map.get(&Pos{x,y:y-1}).unwrap(),xy_node,xy_cost);
            }
            if y < rows - 1
            {
                graph.add_edge(*pos_node_map.get(&Pos{x,y:y+1}).unwrap(),xy_node,xy_cost);
            }
        }
    } 

    let start_node = *pos_node_map.get(&Pos{x:0,y:0}).unwrap();
    let goal_node = *pos_node_map.get(&Pos{x:cols-1,y:rows-1}).unwrap();
    let path = astar(&graph, start_node, 
                     |finish| finish == goal_node, 
                     |e| *e.weight(), 
                     |n| cols-1-graph[n].x + rows-1-graph[n].y );

    println!("{:?}", path);

    Ok(())
}