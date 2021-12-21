use std::io::Error;

fn main() -> Result<(), Error> 
{
    let mut play1_pos = 3;
    let mut play2_pos = 7;

    let mut play1_score = 0;
    let mut play2_score = 0;

    let mut die_value = 1;
    let mut die_rolls = 0;

    let losing_score;

    loop
    {
        for _ in 0..3
        {
            play1_pos = (play1_pos + die_value) % 10;
            die_value = if die_value == 100 { 1 } else { die_value + 1 };
        }
        die_rolls += 3;
        play1_score += play1_pos + 1;
        if play1_score >= 1000
        {
            losing_score = play2_score;
            break;
        }
        for _ in 0..3
        {
            play2_pos = (play2_pos + die_value) % 10;
            die_value = if die_value == 100 { 1 } else { die_value + 1 };
        }
        die_rolls += 3;
        play2_score += play2_pos + 1;
        if play2_score >= 1000 
        {
            losing_score = play1_score;
            break;
        }
    }
    
    println!("{}",die_rolls*losing_score);

    Ok(())
}