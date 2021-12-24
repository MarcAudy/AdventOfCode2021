use std::cmp::{Ord,Ordering};
use std::collections::{BinaryHeap,HashSet}; 
use std::io::Error;

const PART1: bool = false;
const SAMPLE: bool = false;
const ROOM_DEPTH: usize = if PART1 { 2 } else { 4 };
const HALLWAY_LENGTH: usize = 11;

const EMPTY_SPACE: u8 = 4;
const MOVE_COSTS: [i32;4] = [1,10,100,1000];

fn room_to_hallway_index(room_index: usize) -> usize
{
    match room_index
    {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => { assert_eq!(true,false); 111 }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct PuzzleState
{
    hallway: i64,
    rooms: i64,
    cost: i32,
    score: i32,

    goal_rooms: i64,
}

impl Ord for PuzzleState
 {
    fn cmp(&self, other: &Self) -> Ordering 
    {
        if (self.cost + self.score) > (other.cost + other.score) { return Ordering::Less }
        else if (self.cost + self.score) < (other.cost + other.score) { return Ordering::Greater }
        else { return Ordering::Equal; }
    }
}

impl PartialOrd for PuzzleState 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> 
    {
        Some(self.cmp(other))
    }
}

impl PuzzleState
{
    fn new() -> PuzzleState
    {
        PuzzleState 
        { 
            hallway: 0, 
            rooms: 0, 
            cost: 0, 
            score: 0, 
            goal_rooms: 0
        }
    }

    fn get_hallway(&self, hindex: usize) -> u8
    {
        let shift = (hindex * 3) as i64;
        return ((self.hallway & (7 << shift)) >> shift) as u8;
    }

    fn get_room(&self, rindex: usize, rdepth: usize) -> u8
    {
        let shift = ((rindex * ROOM_DEPTH + rdepth) * 3) as i64;
        return ((self.rooms & (7 << shift)) >> shift) as u8;
    }

    fn set_hallway(&mut self, hindex: usize, hvalue: u8)
    {
        let shift = (hindex * 3) as i64;
        self.hallway = (self.hallway & !(7 << shift)) | ((hvalue as i64) << shift);
    }

    fn set_room(&mut self, rindex: usize, rdepth: usize, rvalue: u8)
    {
        let shift = ((rindex * ROOM_DEPTH + rdepth) * 3) as i64;
        self.rooms = (self.rooms & !(7 << shift)) | ((rvalue as i64) << shift);
    }

    fn move_to_hallway(&mut self, rindex: usize, rdepth: usize, hindex: usize)
    {
        let rvalue = self.get_room(rindex, rdepth);

        self.score -= self.score_room(rindex, rdepth);
        self.set_hallway(hindex, rvalue);
        self.set_room(rindex, rdepth, EMPTY_SPACE);
        self.score += self.score_hallway(hindex);

        self.cost += MOVE_COSTS[rvalue as usize] * (rdepth as i32 + 1 + (room_to_hallway_index(rindex) as i32 - hindex as i32).abs()) as i32;
    }

    fn move_to_room(&mut self, hindex: usize, rindex: usize, rdepth: usize)
    {
        let hvalue = self.get_hallway(hindex);

        self.score -= self.score_hallway(hindex);
        self.set_room(rindex, rdepth, hvalue);
        self.set_hallway(hindex, EMPTY_SPACE);

        self.cost += MOVE_COSTS[hvalue as usize] * (rdepth as i32 + 1 + (room_to_hallway_index(rindex) as i32 - hindex as i32).abs()) as i32;
    }

    fn score_hallway(&self, hindex: usize) -> i32
    {
        let target_room = self.get_hallway(hindex);
        if  target_room != EMPTY_SPACE
        {
            return ((room_to_hallway_index(target_room as usize) as i32 - hindex as i32).abs() + 1) * MOVE_COSTS[target_room as usize];
        }

        return 0;
    }

    fn score_room(&self, rindex: usize, rdepth: usize) -> i32
    {
        let target_room = self.get_room(rindex, rdepth) as usize;
        if target_room != EMPTY_SPACE as usize && target_room != rindex
        {
            return (rdepth as i32 + 2 + (room_to_hallway_index(target_room) as i32 - room_to_hallway_index(rindex) as i32).abs()) * MOVE_COSTS[target_room];
        }

        return 0;
    }

    fn from_hallway(hallway: [u8;HALLWAY_LENGTH]) -> i64
    {
        let mut state: PuzzleState = PuzzleState::new();
        for hindex in 0..HALLWAY_LENGTH
        {
            state.set_hallway(hindex, hallway[hindex]);
        }

        return state.hallway;
    }

    fn from_rooms<const COUNT: usize>(rooms: [[u8;COUNT];4]) -> i64
    {
        let mut state: PuzzleState = PuzzleState::new();
        for rindex in 0..4
        {
            for rdepth in 0..ROOM_DEPTH
            {
                state.set_room(rindex, rdepth, rooms[rindex][rdepth]);
            }
        }

        return state.rooms;
    }

    fn is_goal(&self) -> bool
    {
        return self.rooms == self.goal_rooms;
    }

    #[allow(dead_code)]
    fn print(&self)
    {
        let visualize_space = |v|
        {
            match v
            {
                EMPTY_SPACE => '.',
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                _ => '?'
            }
        };

        println!("#############");
        print!("#");
        for hindex in 0..HALLWAY_LENGTH { print!("{}", visualize_space(self.get_hallway(hindex)))}
        println!("#");
        println!("###{}#{}#{}#{}###", 
                 visualize_space(self.get_room(0,0)),
                 visualize_space(self.get_room(1,0)),
                 visualize_space(self.get_room(2,0)),
                 visualize_space(self.get_room(3,0)));
        for rdepth in 1..ROOM_DEPTH
        {
            println!("  #{}#{}#{}#{}#", 
                visualize_space(self.get_room(0,rdepth)),
                visualize_space(self.get_room(1,rdepth)),
                visualize_space(self.get_room(2,rdepth)),
                visualize_space(self.get_room(3,rdepth)));
        }
        println!("  #########");
    }
}

fn main() -> Result<(), Error> 
{
    let initial_hallway: i64 = PuzzleState::from_hallway([EMPTY_SPACE; HALLWAY_LENGTH]);
    let initial_rooms: i64;
    if PART1
    {
        initial_rooms = PuzzleState::from_rooms(
            if SAMPLE { [[1,0],[2,3],[1,2],[3,0]] }
            else      { [[3,2],[3,2],[0,1],[0,1]] });
    }
    else
    {
        initial_rooms = PuzzleState::from_rooms(
            if SAMPLE { [[1,3,3,0],[2,2,1,3],[1,1,0,2],[3,0,2,0]] }
            else      { [[3,3,3,2],[3,2,1,2],[0,1,0,1],[0,0,2,1]] });
    }
    let mut initial_puzzle_state = PuzzleState::new();
    initial_puzzle_state.hallway = initial_hallway;
    initial_puzzle_state.rooms = initial_rooms;
    initial_puzzle_state.goal_rooms = PuzzleState::from_rooms([[0;ROOM_DEPTH],[1;ROOM_DEPTH],[2;ROOM_DEPTH],[3;ROOM_DEPTH]]);

    let mut seen_puzzle_states: HashSet<(i64,i64)> = HashSet::new();
    let mut puzzle_states: BinaryHeap<PuzzleState> = BinaryHeap::new();

    seen_puzzle_states.insert((initial_hallway, initial_rooms));
    puzzle_states.push(initial_puzzle_state);    

    loop
    {
        let cur_state = puzzle_states.pop().unwrap();

        if cur_state.is_goal()
        {
            println!("{}", cur_state.cost);
            break;
        }

        // Exit a room
        for rindex in 0..4
        {
            for rdepth in 0..ROOM_DEPTH
            {
                if cur_state.get_room(rindex, rdepth) != EMPTY_SPACE
                {
                    if (rdepth..ROOM_DEPTH).into_iter().any(|rd| cur_state.get_room(rindex, rd) as usize != rindex)
                    {
                        let hallway_start = room_to_hallway_index(rindex);
                        let mut eval_hallway = |hallway_range: Vec<usize>|
                        {
                            for hindex in hallway_range
                            {
                                if cur_state.get_hallway(hindex) != EMPTY_SPACE
                                {
                                    break;
                                }
                                if hindex == 2 || hindex == 4 || hindex == 6 || hindex == 8
                                {
                                    continue;
                                }
                
                                let mut next_state = cur_state;
                                next_state.move_to_hallway(rindex, rdepth, hindex);
                                if !seen_puzzle_states.contains(&(next_state.hallway, next_state.rooms))
                                {
                                    seen_puzzle_states.insert((next_state.hallway, next_state.rooms));
                                    puzzle_states.push(next_state);
                                }
                            }
                        };
            
                        eval_hallway((hallway_start+1..HALLWAY_LENGTH).collect());
                        eval_hallway((0..hallway_start).rev().collect());
                    }
                    break;
                }
            }
        }

        // Return moves
        for hindex in 0..HALLWAY_LENGTH
        {
            let target_room = cur_state.get_hallway(hindex);
            if target_room != EMPTY_SPACE
            {
                for rdepth in (0..ROOM_DEPTH).rev()
                {
                    let rvalue = cur_state.get_room(target_room as usize,rdepth);
                    if rvalue == EMPTY_SPACE
                    {
                        let clear;
    
                        let target_hallway = room_to_hallway_index(target_room as usize);
                        if target_hallway > hindex
                        {
                            clear = !(hindex+1..=target_hallway).into_iter().any(|n| cur_state.get_hallway(n) != EMPTY_SPACE);
                        }
                        else
                        {
                            clear = !(target_hallway..hindex).into_iter().any(|n| cur_state.get_hallway(n) != EMPTY_SPACE);
                        }
                        if clear
                        {                           
                            let mut next_state = cur_state;
                            next_state.move_to_room(hindex, target_room as usize, rdepth);
                            if !seen_puzzle_states.contains(&(next_state.hallway, next_state.rooms))
                            {
                                seen_puzzle_states.insert((next_state.hallway, next_state.rooms));
                                puzzle_states.push(next_state);
                            }
                        }

                        break;
                    }
                    else if rvalue != target_room
                    {
                        break;                        
                    }
                }
            }
        }
    }


    Ok(())
}