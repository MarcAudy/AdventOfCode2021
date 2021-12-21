use std::collections::HashMap;
use std::io::Error;

const INCREMENTS: [u8;7] = [3,4,5,6,7,8,9];
const MULTIPLIERS: [i64;7] = [1,3,6,7,6,3,1];

fn main() -> Result<(), Error> 
{
    let mut universes: HashMap<(u8,u8,u8,u8,bool),i64> = HashMap::new();   

    let mut play1_wins = 0;
    let mut play2_wins = 0;

    universes.insert((3,0,7,0,true),1);

    while universes.len() > 0
    {
        let upair = universes.iter_mut().next().unwrap();
        let universe = *upair.0;
        let old_universe_count = *upair.1;
        universes.remove(&universe);

        for i in 0..7
        {
            if universe.4
            {
                let new_pos = (universe.0 + INCREMENTS[i]) % 10;
                let new_score = universe.1 + new_pos + 1;
                let new_universe_count = old_universe_count * MULTIPLIERS[i];
                if new_score >= 21
                {
                    play1_wins += new_universe_count;
                }
                else
                {
                    *universes.entry((new_pos, new_score, universe.2, universe.3, false)).or_insert(0) += new_universe_count;
                }
            }
            else
            {
                let new_pos = (universe.2 + INCREMENTS[i]) % 10;
                let new_score = universe.3 + new_pos + 1;
                let new_universe_count = old_universe_count * MULTIPLIERS[i];
                if new_score >= 21
                {
                    play2_wins += new_universe_count;
                }
                else
                {
                    *universes.entry((universe.0, universe.1, new_pos, new_score, true)).or_insert(0) += new_universe_count;
                }
            }
        }
    }
    

    println!("{:?}",[play1_wins,play2_wins].iter().max());

    Ok(())
}