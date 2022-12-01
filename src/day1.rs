use std::cmp::max;
use std::error::Error;
use std::fs;

pub fn day1a() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("inputs/day1a.txt")?;
    let mut sum = 0;
    let mut best = 0;
    let mut idx = 0;
    let mut best_idx = -1;
    for line in text.lines() {
        if line.len() == 0 {
            best = max(sum, best);
            best_idx = idx;
            sum = 0;
            idx += 1;
        } else {
            let datum = line.parse::<i32>()?;
            sum += datum;
        }
    }
    println!("The elf carrying the most calories is elf {best_idx}, with {best} calories.");
    Ok(())
}